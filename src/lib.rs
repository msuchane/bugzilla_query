// Bugzilla API documentation:
// https://bugzilla.redhat.com/docs/en/html/api/core/v1/general.html

use log::debug;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

mod bug_model;

pub use crate::bug_model::{Bug, BugzillaError, Flag, Response, User};

// TODO: Make these configurable.
// For now, let's define the included fields as a constant.
const INCLUDED_FIELDS: &str = "_default,pool,flags";

/// The authorization method that the crate uses when contacting Bugzilla.
pub enum Authorization {
    Anonymous,
    ApiKey(String),
}

/// Access a single bug by its ID.
pub fn bug(host: &str, id: &str, auth: Authorization) -> Result<Bug, Error> {
    // Reuse the `bugs` function. Later, extract the first element.
    let bugs = bugs(host, &[id], auth)?;

    // This is a way to return the first (and only) element of the Vec,
    // without cloning it.
    // TODO: I'm using InvalidValue here mostly as a placeholder.
    // The response should always contain one bug, but if it doesn't,
    // I don't know how best to report it. Maybe just panic?
    bugs.into_iter().next().ok_or(Error::InvalidValue)
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

/// Access several bugs by their IDs.
pub fn bugs(host: &str, ids: &[&str], auth: Authorization) -> Result<Vec<Bug>, Error> {
    let mut client = RestClient::builder().blocking(host)?;

    // If the user selects the API key authorization, set the API key in the request header.
    // Otherwise, the anonymous authorization doesn't modify the request in any way.
    if let Authorization::ApiKey(key) = auth {
        client.set_header("Authorization", &format!("Bearer {}", key))?;
    }

    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<Response> = client.get(ids)?;
    let response = data.into_inner();
    debug!("{:#?}", response);

    // TODO: Note that the resulting list might be empty and still Ok
    Ok(response.bugs)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
