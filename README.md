# bugzilla_query

[![Crates.io](https://img.shields.io/crates/v/bugzilla_query.svg)](https://crates.io/crates/bugzilla_query)
[![Apache-2.0 license](https://img.shields.io/crates/l/bugzilla_query)](https://crates.io/crates/bugzilla_query)
[![Documentation](https://docs.rs/bugzilla_query/badge.svg)](https://docs.rs/bugzilla_query)

[![CI tests](https://github.com/msuchane/bugzilla_query/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/msuchane/bugzilla_query/actions/workflows/rust-tests.yml)
[![Dependency status](https://deps.rs/repo/github/msuchane/bugzilla_query/status.svg)](https://deps.rs/repo/github/msuchane/bugzilla_query)

Access bugs on a remote Bugzilla instance.

## Description

The `bugzilla_query` crate is a Rust library that can query a Bugzilla instance using its REST API. It returns a strongly typed representation of the requested bugs.

This library provides no functionality to create or modify bugs. The access is read-only.

## Usage

### Basic anonymous query

Without logging in, search for a single bug and check for its assignee:

```rust
use tokio;
use bugzilla_query::BzInstance;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bugzilla = BzInstance::at("https://bugzilla.redhat.com".to_string())?;

    let bug = bugzilla.bug("1906883").await?;

    assert_eq!(bug.assigned_to, "Marek Suchánek");

    Ok(())
}
```

### Advanced query

Use an API key to log into Bugzilla. Search for all bugs on Fedora 36 that belong to the `rust` component. Check that there is more than one bug:

```rust
use tokio;
use bugzilla_query::{Auth, BzInstance, Pagination};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let bugzilla = BzInstance::at("https://bugzilla.redhat.com".to_string())?
        .authenticate(Auth::ApiKey("My API Key".to_string()))
        .paginate(Pagination::Unlimited);

    let query = "component=rust&product=Fedora&version=36";

    let bugs = bugzilla.search(query).await?;

    assert!(bugs.len() > 1);

    Ok(())
}
```

## See also

* [`jira_query`](https://crates.io/crates/jira_query), a similar interface to Jira
