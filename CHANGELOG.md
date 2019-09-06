#  Procedure for Making a Release

 - Set correct version number in Cargo.toml
 - Update CHANGELOG.md
 - Commit changes and push to Github
 - Run "cargo test"
   - If it fails, fix and restart release procedure
 - Run "cargo publish --dry-run"
   - If it fails, fix and restart release procedure
 - Tag the release in Github
 - Run "cargo publish" to push to crates.io
 - Run "cargo download ietfdata-rs==0.2.4 -o releases/ietfdata-rs-0.2.4.crate",
   replacing both instances of 0.2.4 with the latest version number, to
   download the crate.
 - Check the downloaded and locally built crates are identical:
     shasum -a 256 releases/ietfdata-rs-0.2.4.crate
     shasum -a 256 target/package/ietfdata-rs-0.2.4.crate
 - Add the downloaded crate to git and push the repo to Github
  


# Change Log

## v0.3.0 -- 2019-08-19

 - Revise `PaginatedList` and related types to return `Result<>`
 - Rename `email()` method to `email_from_address()` and add
   a replacement `email()` method that takes an `EmailUri`
 - Rename `person_from_email()` to `person_from_email_address()`
   and add a replacement `person_from_email()` method that takes
   an `EmailUri`
 - Add new types:
    - `GroupUri`        and `Group`
    - `GroupTypeUri`    and `GroupType`
    - `GroupStateUri`   and `GroupState`
    - `DocumentUri`     and `Document`
    - `SubmissionUri`   and `Submission`
    - `DocStateUri`     and `DocState`
    - `DocStateTypeUri` and `DocStateType`
 - Add new methods:
    - `doc_state_type()`
    - `doc_state_types()`



## v0.2.4 -- 2019-08-18

 - Rename 'Alias' to 'PersonAlias' since there will be document aliases
   added later.


## v0.2.3 -- 2019-08-18

 - Add new types:
    - `Alias`
    - `HistoricalPerson`
    - `HistoricalEmail`
 - Add new methods:
    - `people_between()`
    - `person_aliases()`
    - `person_history()`
    - `email_history_for_address()` 
    - `email_history_for_person()`


## v0.2.2 -- 2019-08-17

 - Add new methods:
    - `people_with_name()`
    - `people_with_name_containing()`


## v0.2.1 -- 2019-08-17

 - Add new method:
    - `person_from_email()`


## v0.2.0 -- 2019-08-16

 - Update API to return custom error and not panic!() on failure.


## v0.1.1 -- 2019-08-10

 - Add metadata


## v0.1.0 -- 2019-08-10

 - Initial version, implementing the following methods:
    - `email()`
    - `person()`
    - `people()`

