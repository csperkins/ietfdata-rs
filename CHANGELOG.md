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
 - Run "cargo download ietfdata-rs==0.1.1 > releases/ietfdata-rs-0.1.1.crate",
   replacing 0.1.1 with the latest version number, to download the crate. Add
   to downloaded file to the git repo and push.
  


# Change Log

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

