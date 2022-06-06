// use restson;
use bugzilla_query::*;

/// Try accessing a public bug intended for testing
#[test]
fn access_bug() {
    let _bug = bug(
        "https://bugzilla.redhat.com",
        "1906887",
        Authorization::Anonymous,
    )
    .unwrap();
}
