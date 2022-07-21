// use restson;
use chrono::prelude::*;
use tokio;

use bugzilla_query::*;
use serde_json::Value;

/// A common convenience function to get anonymous access
/// to the Red Hat Bugzilla instance.
fn rh_bugzilla() -> BzInstance {
    BzInstance::at("https://bugzilla.redhat.com".to_string()).unwrap()
}

/// Try accessing a public bug intended for testing
#[tokio::test]
async fn access_bug() {
    let instance = rh_bugzilla();
    let _bug = instance.bug("1906883").await.unwrap();
}

/// Try accessing a bug that doesn not exist.
#[tokio::test]
async fn access_missing_bug() {
    let instance = rh_bugzilla();
    let bug = instance.bug("111111111111111111");

    assert!(matches!(bug.await.unwrap_err(), BugzillaQueryError::NoBugs));
}

/// Check that the bug fields contain the expected values.
/// Work with fields that are standard in Bugzilla, rather than custom extensions.
#[tokio::test]
async fn check_standard_fields() {
    let instance = rh_bugzilla();
    let bug = instance.bug("1906887").await.unwrap();

    assert_eq!(bug.id, 1906887);
    assert_eq!(
        bug.summary,
        "Test the CoRN release notes generator (populated)"
    );
    assert_eq!(bug.status, "CLOSED");
    assert_eq!(bug.resolution, "CURRENTRELEASE");
    assert_eq!(bug.is_open, false);
    assert_eq!(bug.component[0], "Documentation");
    assert_eq!(bug.priority, "medium");
    assert_eq!(bug.assigned_to, "Marek Suchánek");
    assert_eq!(bug.assigned_to_detail.email, "msuchane");
    assert_eq!(bug.docs_contact, "Marek Suchánek");
    assert_eq!(bug.docs_contact_detail.unwrap().email, "msuchane");
}

/// Check that the bug was created at the expected date, and that time deserialization
/// works as expected.
#[tokio::test]
async fn check_time() {
    let instance = rh_bugzilla();
    let bug = instance.bug("1906887").await.unwrap();

    let date_created = chrono::Utc.ymd(2020, 12, 11);
    assert_eq!(bug.creation_time.date(), date_created);
}

/// Check that the bug fields contain the expected values.
/// Work with custom fields that are available in the Red Hat Bugzilla.
/// Namely, check access to the Doc Text field.
#[tokio::test]
async fn check_custom_fields() {
    let instance = rh_bugzilla();
    let bug = instance.bug("1906887").await.unwrap();

    // As a custom field, Doc Text is available in the `extra` hash map.
    let doc_text = bug.extra.get("cf_release_notes").and_then(Value::as_str);

    // This is the expected value of Doc Text. Bugzilla uses `\r\n` line endings.
    let release_note = ".A test\r\n\
        \r\n\
        This is a testing release note.\r\n\
        \r\n\
        It is written in the proper format.\r\n\
        \r\n\
        The following is a list:\r\n\
        \r\n\
        * One\r\n\
        * Two\r\n\
        * Three";

    assert_eq!(doc_text, Some(release_note));
}

// Access to flags requires authentication, so I'm disabling this test for now.
// TODO: Enable authenticated tests.
/*
/// Check that we can access flags and that a selected flag has the expected value.
#[test]
fn check_flags() {
    let instance = rh_bugzilla();
    let bug = instance.bug("1906887").unwrap();

    let rdt = bug.get_flag("requires_doc_text");

    assert_eq!(rdt, Some("+"));
} */

/// Try accessing bugs that match a Bugzilla search query.
#[tokio::test]
async fn search_for_bugs() {
    let instance = rh_bugzilla();
    let query = "component=rust&product=Fedora&version=36";
    let _bugs = instance.search(query).await.unwrap();
}

/// Make sure that no IDs on the input result in no bugs, without errors.
#[tokio::test]
async fn check_no_bugs() {
    let instance = rh_bugzilla();
    let bugs = instance.bugs(&[]).await;

    assert_eq!(bugs.ok(), Some(vec![]));
}
