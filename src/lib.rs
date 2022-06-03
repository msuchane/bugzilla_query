// API documentation:
// https://bugzilla.redhat.com/docs/en/html/api/core/v1/general.html

use log::debug;
use restson::{Error, Response as RestResponse, RestClient, RestPath};

mod bug_model;

pub use crate::bug_model::{Bug, BugzillaError, Response};

// API call with one &str parameter, which is the bug ID
impl RestPath<&str> for Response {
    fn get_path(param: &str) -> Result<String, Error> {
        // TODO: Make these configurable:
        Ok(format!("rest/bug?id={}&include_fields=_default,pool,flags", param))
    }
}

pub fn bug(host: &str, bug: &str, api_key: &str) -> Result<Bug, Error> {
    let mut client = RestClient::builder().blocking(host)?;
    client.set_header("Authorization", &format!("Bearer {}", api_key))?;
    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<Response> = client.get(bug)?;
    let response = data.into_inner();
    debug!("{:#?}", response);

    // This is a way to return the first (and only) element of the Vec,
    // without cloning it.
    // TODO: I'm using InvalidValue here mostly as a placeholder.
    // The response should always contain one bug, but if it doesn't,
    // I don't know how best to report it. Maybe just panic?
    response.bugs.into_iter().next().ok_or(Error::InvalidValue)
}

// API call with several &str parameter, which are the bug IDs.
// TODO: Make this generic over &[&str] and &[String].
impl RestPath<&[&str]> for Response {
    fn get_path(params: &[&str]) -> Result<String, Error> {
        // TODO: Make these configurable:
        Ok(format!("rest/bug?id={}&include_fields=_default,pool,flags", params.join(",")))
    }
}

pub fn bugs(host: &str, bugs: &[&str], api_key: &str) -> Result<Vec<Bug>, Error> {
    let mut client = RestClient::builder().blocking(host)?;
    client.set_header("Authorization", &format!("Bearer {}", api_key))?;
    // Gets a bug by ID and deserializes the JSON to data variable
    let data: RestResponse<Response> = client.get(bugs)?;
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
