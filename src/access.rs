/*
Copyright 2022 Marek Suchánek <msuchane@redhat.com>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Bugzilla API documentation:
// https://bugzilla.redhat.com/docs/en/html/api/core/v1/general.html

use crate::bug_model::{Bug, Response};
use crate::errors::BugzillaQueryError;

/// Configuration and credentials to access a Bugzilla instance.
pub struct BzInstance {
    pub host: String,
    pub auth: Auth,
    pub pagination: Pagination,
    pub included_fields: Vec<String>,
    client: reqwest::Client,
}

/// The authentication method that the crate uses when contacting Bugzilla.
pub enum Auth {
    Anonymous,
    ApiKey(String),
    Basic { user: String, password: String },
}

// We could set a default enum variant and derive, but that raises the MSRV to 1.62.
impl Default for Auth {
    fn default() -> Self {
        Self::Anonymous
    }
}

/// Controls the upper limit of how many bugs the response from Bugzilla can contain:
///
/// * `Default`: Use the default settings of this instance, which sets an arbitrary limit on the number of bugs.
/// * `Limit`: Use this upper limit instead.
/// * `Unlimited`: Set the limit to 0, which disables the upper limit and returns all matching bugs.
pub enum Pagination {
    Default,
    Limit(u32),
    Unlimited,
}

// We could set a default enum variant and derive, but that raises the MSRV to 1.62.
impl Default for Pagination {
    fn default() -> Self {
        Self::Default
    }
}

impl Pagination {
    /// Format the `Pagination` variant as a URL query fragment, such as `?limit=20`.
    fn url_fragment(&self) -> String {
        match self {
            Pagination::Default => String::new(),
            Pagination::Limit(n) => format!("&limit={n}"),
            Pagination::Unlimited => "&limit=0".to_string(),
        }
    }
}

/// The method of the request to Bugzilla. Either request specific IDs,
/// or use a free-form Bugzilla search query as-is.
enum Method<'a> {
    Ids(&'a [&'a str]),
    Search(&'a str),
}

impl<'a> Method<'a> {
    fn url_fragment(&self) -> String {
        match self {
            Self::Ids(ids) => format!("id={}", ids.join(",")),
            Self::Search(query) => (*query).to_string(),
        }
    }
}

impl BzInstance {
    /// Create a new `BzInstance` struct using a host URL, with default values
    /// for all options.
    pub fn at(host: String) -> Result<Self, BugzillaQueryError> {
        // TODO: This function takes host as a String, even though client is happy with &str.
        // The String is only used in the host struct attribute.

        let client = reqwest::Client::new();

        Ok(BzInstance {
            host,
            client,
            included_fields: vec!["_default".to_string()],
            auth: Auth::default(),
            pagination: Pagination::default(),
        })
    }

    /// Set the authentication method of this `BzInstance`.
    #[must_use]
    pub fn authenticate(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    /// Set the pagination method of this `BzInstance`.
    #[must_use]
    pub fn paginate(mut self, pagination: Pagination) -> Self {
        self.pagination = pagination;
        self
    }

    /// Set Bugzilla fields that this `BzInstance` will request, such as `flags`.
    ///
    /// By default, `BzInstance` requests the `_default` fields, and using this method
    /// overwrites the default value. If you want to set fields in addition
    /// to `_default`, specify `_default` in your list.
    #[must_use]
    pub fn include_fields(mut self, fields: Vec<String>) -> Self {
        self.included_fields = fields;
        self
    }

    /// Format the included Bugzilla fields as a URL query fragment, such as `&include_fields=_default,flags`.
    #[must_use]
    fn fields_as_query(&self) -> String {
        if self.included_fields.is_empty() {
            String::new()
        } else {
            format!("&include_fields={}", self.included_fields.join(","))
        }
    }

    /// Based on the request method, form a complete, absolute URL
    /// to download the tickets from the REST API.
    #[must_use]
    fn path(&self, method: &Method) -> String {
        format!(
            "{}/rest/bug?{}{}{}",
            &self.host,
            method.url_fragment(),
            self.fields_as_query(),
            self.pagination.url_fragment()
        )
    }

    /// Download the specified URL using the configured authentication.
    async fn authenticated_get(&self, url: &str) -> Result<reqwest::Response, reqwest::Error> {
        let request_builder = self.client.get(url);
        let authenticated = match &self.auth {
            Auth::Anonymous => request_builder,
            Auth::ApiKey(key) => request_builder.header("Authorization", &format!("Bearer {key}")),
            Auth::Basic { user, password } => request_builder.basic_auth(user, Some(password)),
        };
        authenticated.send().await
    }

    /// Access several bugs by their IDs.
    pub async fn bugs(&self, ids: &[&str]) -> Result<Vec<Bug>, BugzillaQueryError> {
        // If the user specifies no IDs, skip network requests and return no bugs.
        // Returning an error could also be valid, but I believe that this behavior
        // is less surprising and more practical.
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let url = self.path(&Method::Ids(ids));

        // Gets a bug by ID and deserializes the JSON to data variable
        let response = self
            .authenticated_get(&url)
            .await?
            .json::<Response>()
            .await?;

        log::debug!("{:#?}", response);

        // The resulting list might be empty. In that case, return an error.
        if response.bugs.is_empty() {
            Err(BugzillaQueryError::NoBugs)
        } else {
            Ok(response.bugs)
        }
    }

    /// Access a single bug by its ID.
    pub async fn bug(&self, id: &str) -> Result<Bug, BugzillaQueryError> {
        // Reuse the `bugs` function. Later, extract the first element.
        let bugs = self.bugs(&[id]).await?;

        // This is a way to return the first (and only) element of the Vec,
        // without cloning it.
        bugs.into_iter().next().ok_or(BugzillaQueryError::NoBugs)
    }

    /// Access bugs using a free-form Bugzilla search query.
    ///
    /// An example of a query: `component=rust&product=Fedora&version=36`.
    pub async fn search(&self, query: &str) -> Result<Vec<Bug>, BugzillaQueryError> {
        let url = self.path(&Method::Search(query));

        // Gets the bugs by query and deserializes the JSON to data variable
        let response = self
            .authenticated_get(&url)
            .await?
            .json::<Response>()
            .await?;

        log::debug!("{:#?}", response);

        // The resulting list might be empty. In that case, return an error.
        if response.bugs.is_empty() {
            Err(BugzillaQueryError::NoBugs)
        } else {
            Ok(response.bugs)
        }
    }
}
