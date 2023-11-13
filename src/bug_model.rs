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

//! This module replicates the fields in a Bugzilla bug as strongly typed structs.
//! Any extra fields that come from a custom Bugzilla configuration are captured
//! in the `extra` hash map in the parent struct.

use std::fmt;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::Value;

/// The response from Bugzilla, which includes the list of requested bugs
/// and some additional metadata.
#[derive(Clone, Debug, Deserialize)]
pub struct Response {
    pub offset: Option<u32>,
    pub limit: Option<String>,
    pub total_matches: Option<u32>,
    pub bugs: Vec<Bug>,
    #[serde(flatten)]
    pub extra: Value,
}

/// An error report from Bugzilla.
#[derive(Clone, Debug, Deserialize)]
pub struct BugzillaError {
    pub error: bool,
    pub message: String,
    pub code: i32,
    #[serde(flatten)]
    pub extra: Value,
}

/// Certain fields can appear as a single, optional string or a list of strings based on the Bugzilla instance and its configuration.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum OneOrMany {
    None,
    One(String),
    Many(Vec<String>),
}

impl OneOrMany {
    /// Regardless of the Bugzilla instance configuration, list all items in a vector: empty, one item, or more items.
    pub fn into_vec(self) -> Vec<String> {
        match self {
            Self::None => Vec::new(),
            Self::One(s) => vec![s],
            Self::Many(v) => v,
        }
    }
}

/// Some Bugzilla instances set the component as a single string, some use a list of components.
pub type Component = OneOrMany;

/// Some Bugzilla instances set the version as a single string, some use a list of versions.
pub type Version = OneOrMany;

/// Some Bugzilla instances set the alias as a single string, some use a list of aliases.
pub type Alias = OneOrMany;

/// The representation of a single Bugzilla bug with all its fields.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Bug {
    pub alias: Alias,
    pub op_sys: String,
    pub classification: String,
    pub id: i32,
    pub url: String,
    pub creator: String,
    pub creator_detail: User,
    pub summary: String,
    pub status: String,
    pub estimated_time: Option<i64>,
    pub target_milestone: String,
    pub cc: Vec<String>,
    pub cc_detail: Vec<User>,
    pub is_open: bool,
    pub is_creator_accessible: bool,
    pub docs_contact: Option<String>,
    pub docs_contact_detail: Option<User>,
    pub assigned_to: String,
    pub assigned_to_detail: User,
    pub resolution: String,
    pub severity: String,
    pub product: String,
    pub platform: String,
    pub last_change_time: DateTime<Utc>,
    pub remaining_time: Option<i64>,
    pub priority: String,
    pub whiteboard: String,
    pub creation_time: DateTime<Utc>,
    pub is_confirmed: bool,
    pub qa_contact: String,
    pub qa_contact_detail: Option<User>,
    pub dupe_of: Option<i32>,
    pub target_release: Option<Version>,
    pub actual_time: Option<i64>,
    pub component: Component,
    pub is_cc_accessible: bool,
    pub version: Version,
    pub keywords: Vec<String>,
    pub depends_on: Vec<i32>,
    pub blocks: Vec<i32>,
    pub see_also: Option<Vec<String>>,
    pub groups: Vec<String>,
    /// Bugzilla stores `deadline` only as `YYYY-MM-DD`, so it can't deserialize to full `DateTime`.
    pub deadline: Option<NaiveDate>,
    pub update_token: Option<String>,
    pub work_time: Option<i64>,
    // Not part of the default response:
    pub flags: Option<Vec<Flag>>,
    pub tags: Option<Vec<String>>,
    pub dependent_products: Option<Vec<String>>,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a Bugzilla user account.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct User {
    pub email: String,
    pub id: i32,
    pub name: String,
    pub real_name: String,
    #[serde(flatten)]
    pub extra: Value,
}

/// The representation of a flag in a bug.
/// A flag resembles a hash map entry, where `flag.name` is the key
/// and `flag.status` is the value.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Flag {
    pub id: i32,
    pub type_id: i32,
    pub creation_date: DateTime<Utc>,
    pub modification_date: DateTime<Utc>,
    pub name: String,
    pub status: String,
    pub setter: String,
    pub requestee: Option<String>,
    #[serde(flatten)]
    pub extra: Value,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Displays the flag in the format of `name: value`.
        write!(f, "{}: {}", self.name, self.status)
    }
}
