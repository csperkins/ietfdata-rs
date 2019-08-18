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
  


# Change Log

## v???

 - Add `people_between()` method


## v0.2.2 -- 2019-08-17

 - Add `people_with_name()` method
 - Add `people_with_name_containing()` method


## v0.2.1 -- 2019-08-17

 - Add `person_from_email()` method


## v0.2.0 -- 2019-08-16

 - Update API to return custom error and not panic!() on failure.


## v0.1.1 -- 2019-08-10

 - Add metadata


## v0.1.0 -- 2019-08-10

 - Initial version, with email(), person(), and people() methods.

