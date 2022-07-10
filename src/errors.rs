use thiserror::Error;

/// All errors that might occur in this crate.
#[derive(Error, Debug)]
pub enum BugzillaQueryError {
    #[error("Required bugs are missing in the Bugzilla response: {}.", .0.join(", "))]
    MissingBugs(Vec<String>),
    #[error("The Bugzilla query returned no bugs.")]
    NoBugs,
    #[error("Error in the Bugzilla REST API.")]
    Rest(#[from] restson::Error),
}
