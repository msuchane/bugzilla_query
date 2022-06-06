// Bugzilla API documentation:
// https://bugzilla.redhat.com/docs/en/html/api/core/v1/general.html

use log::debug;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

use crate::bug_model::{Bug, Response};

/// Configuration and credentials to access a Bugzilla instance.
pub struct BzInstance {
    pub host: String,
    pub auth: Auth,
}

// TODO: Make these configurable.
// For now, let's define the included fields as a constant.
const INCLUDED_FIELDS: &str = "_default,pool,flags";

/// The authentication method that the crate uses when contacting Bugzilla.
pub enum Auth {
    Anonymous,
    ApiKey(String),
}

// TODO: Make this generic over &[&str] and &[String].
/// API call with several &str parameter, which are the bug IDs.
impl RestPath<&[&str]> for Response {
    fn get_path(params: &[&str]) -> Result<String, Error> {
        // TODO: Make these configurable:
        Ok(format!(
            "rest/bug?id={}&include_fields={}",
            params.join(","),
            INCLUDED_FIELDS
        ))
    }
}

impl BzInstance {
    /// Access several bugs by their IDs.
    pub fn bugs(&self, ids: &[&str]) -> Result<Vec<Bug>, Error> {
        let mut client = RestClient::builder().blocking(&self.host)?;

        // If the user selects the API key authorization, set the API key in the request header.
        // Otherwise, the anonymous authorization doesn't modify the request in any way.
        if let Auth::ApiKey(key) = &self.auth {
            client.set_header("Authorization", &format!("Bearer {}", key))?;
        }

        // Gets a bug by ID and deserializes the JSON to data variable
        let data: RestResponse<Response> = client.get(ids)?;
        let response = data.into_inner();
        debug!("{:#?}", response);

        // TODO: Note that the resulting list might be empty and still Ok
        Ok(response.bugs)
    }

    /// Access a single bug by its ID.
    pub fn bug(&self, id: &str) -> Result<Bug, Error> {
        // Reuse the `bugs` function. Later, extract the first element.
        let bugs = self.bugs(&[id])?;

        // This is a way to return the first (and only) element of the Vec,
        // without cloning it.
        // TODO: I'm using InvalidValue here mostly as a placeholder.
        // The response should always contain one bug, but if it doesn't,
        // I don't know how best to report it. Maybe just panic?
        bugs.into_iter().next().ok_or(Error::InvalidValue)
    }
}
