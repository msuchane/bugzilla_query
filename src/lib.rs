// Enable additional clippy lints by default.
#![warn(
    clippy::pedantic,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::clone_on_ref_ptr,
    clippy::todo
)]
// Forbid unsafe code in this program.
#![forbid(unsafe_code)]

mod access;
mod bug_methods;
mod bug_model;
mod errors;

pub use access::{Auth, BzInstance, Pagination};
pub use bug_model::{Bug, Component, Flag, User, Version};
pub use errors::BugzillaQueryError;
// Re-export JSON Value because it's an integral part of the bug model.
pub use serde_json::Value;
