// use restson;
use bugzilla_query::*;

/// Try accessing a public bug intended for testing
#[test]
fn access_bug() {
    let _bug = bug(
        "https://bugzilla.redhat.com",
        "1906883",
        Authorization::Anonymous,
    )
    .unwrap();
}

/// Check that the bug fields contain the expected values.
/// Work with fields that are standard in Bugzilla, rather than custom extensions.
#[test]
fn check_standard_fields() {
    let bug = bug(
        "https://bugzilla.redhat.com",
        "1906887",
        Authorization::Anonymous,
    )
    .unwrap();

    assert_eq!(bug.id, 1906887);
    assert_eq!(bug.summary, "Test the CoRN release notes generator (populated)");
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