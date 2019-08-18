// Copyright (C) 2019 University of Glasgow
// 
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions 
// are met:
// 
// 1. Redistributions of source code must retain the above copyright notice,
//    this list of conditions and the following disclaimer.
// 
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
//
// SPDX-License-Identifier: BSD-2-Clause

// This library contains code to interact with the IETF Datatracker
// (https://datatracker.ietf.org/release/about)
//
// The Datatracker API is at https://datatracker.ietf.org/api/v1 and is
// a REST API implemented using Django Tastypie (http://tastypieapi.org)
//
// It's possible to do time range queries on many of these values, for example:
//   https://datatracker.ietf.org/api/v1/person/person/?time__gt=2018-03-27T14:07:36
//
// See also:
//   https://datatracker.ietf.org/api/
//   https://trac.tools.ietf.org/tools/ietfdb/wiki/DatabaseSchemaDescription
//   https://trac.tools.ietf.org/tools/ietfdb/wiki/DatatrackerDrafts
//   RFC 6174 "Definition of IETF Working Group Document States"
//   RFC 6175 "Requirements to Extend the Datatracker for IETF Working Group Chairs and Authors"
//   RFC 6292 "Requirements for a Working Group Charter Tool"
//   RFC 6293 "Requirements for Internet-Draft Tracking by the IETF Community in the Datatracker"
//   RFC 6322 "Datatracker States and Annotations for the IAB, IRTF, and Independent Submission Streams"
//   RFC 6359 "Datatracker Extensions to Include IANA and RFC Editor Processing Information"
//   RFC 7760 "Statement of Work for Extensions to the IETF Datatracker for Author Statistics"

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer};
use std::error;
use std::fmt;

// =================================================================================================================================
// Helper functions:

fn deserialize_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f").map_err(serde::de::Error::custom)
}

// Generic types representing a paginated list of responses from the Datatracker:

#[derive(Deserialize, Debug)]
struct Meta {
    total_count : u32,
    limit       : u32,
    offset      : u32,
    previous    : Option<String>,
    next        : Option<String>
}

#[derive(Deserialize, Debug)]
struct Page<T> {
    meta        : Meta,
    objects     : Vec<T>
}

pub struct PaginatedList<'a, T> {
    iter : <Vec<T> as IntoIterator>::IntoIter,
    next : Option<String>,
    dt   : &'a Datatracker
}

impl<'a, T> PaginatedList<'a, T>
    where for<'de> T: Deserialize<'de>
{
    pub fn new(dt: &'a Datatracker, url : String) -> Self {
        let mut res = dt.connection.get(&url).send().unwrap(); // FIXME
        let pl : Page<T> = res.json().unwrap();                // FIXME

        Self {
            next : pl.meta.next.clone(),
            iter : pl.objects.into_iter(),
            dt   : dt
        }
    }
}

impl<'a, T> Iterator for PaginatedList<'a, T>
    where for<'de> T: Deserialize<'de>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().or_else(||
            match self.next.clone() {
                Some(ref url_frag) => {
                    let url = format!("https://datatracker.ietf.org/{}", url_frag);
                    let mut res = self.dt.connection.get(&url).send().unwrap(); // FIXME
                    let pl : Page<T> = res.json().unwrap();                     // FIXME
                    self.next = pl.meta.next.clone();
                    self.iter = pl.objects.into_iter();
                    self.iter.next()
                }
                None => {
                    None
                }
            }
        )
    }
}

// =================================================================================================================================
// IETF Datatracker types:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct EmailUri(String);

/// A mapping from email address to person in the IETF datatracker.
#[derive(Deserialize, Debug)]
pub struct Email {
    pub resource_uri : EmailUri,
    pub address      : String,
    pub person       : PersonUri,
    #[serde(deserialize_with="deserialize_time")]
    pub time         : DateTime<Utc>,
    pub origin       : String,
    pub primary      : bool,
    pub active       : bool
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct HistoricalEmailUri(String);

#[derive(Deserialize, Debug)]
pub struct HistoricalEmail {
    // Fields common with Email:
    pub resource_uri          : HistoricalEmailUri,
    pub address               : String,
    pub person                : PersonUri,
    #[serde(deserialize_with="deserialize_time")]
    pub time                  : DateTime<Utc>,
    pub origin                : String,
    pub primary               : bool,
    pub active                : bool,
    // Fields recording the history:
    pub history_change_reason : Option<String>,
    pub history_user          : Option<String>,
    pub history_id            : u64,
    pub history_type          : String,
    #[serde(deserialize_with="deserialize_time")]
    pub history_date          : DateTime<Utc>
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PersonUri(String);

/// A person in the IETF datatracker.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Person {
    pub id              : u64,
    pub resource_uri    : PersonUri,
    pub name            : String,
    pub name_from_draft : Option<String>,
    pub biography       : String,
    pub ascii           : String,
    pub ascii_short     : Option<String>,
    #[serde(deserialize_with="deserialize_time")]
    pub time            : DateTime<Utc>,
    pub photo           : Option<String>,  // Actually a URL
    pub photo_thumb     : Option<String>,  // Actually a URL
    pub user            : Option<String>,
    pub consent         : Option<bool>
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct HistoricalPersonUri(String);

/// A historical person in the IETF datatracker.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HistoricalPerson {
    // Fields common with Person:
    pub id                    : u64,
    pub resource_uri          : HistoricalPersonUri,
    pub name                  : String,
    pub name_from_draft       : String,
    pub biography             : String,
    pub ascii                 : String,
    pub ascii_short           : Option<String>,
    #[serde(deserialize_with="deserialize_time")]
    pub time                  : DateTime<Utc>,
    pub photo                 : Option<String>, // Actually a URL
    pub photo_thumb           : Option<String>, // Actually a URL
    pub user                  : String,
    pub consent               : Option<bool>,
    // Fields recording the history:
    pub history_change_reason : Option<String>,
    pub history_user          : String,
    pub history_type          : String,
    pub history_id            : u64,
    #[serde(deserialize_with="deserialize_time")]
    pub history_date          : DateTime<Utc>,
}

// =================================================================================================================================
// The DatatrackerError type:

#[derive(Debug)]
pub enum DatatrackerError {
    NotFound,
    IoError(reqwest::Error)
}

impl fmt::Display for DatatrackerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatatrackerError::NotFound => write!(f, "Not found"),
            DatatrackerError::IoError(ref e) => e.fmt(f)
        }
    }
}

impl error::Error for DatatrackerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DatatrackerError::NotFound => None,
            DatatrackerError::IoError(ref e) => Some(e)
        }
    }
}

impl From<reqwest::Error> for DatatrackerError {
    fn from(err: reqwest::Error) -> DatatrackerError {
        DatatrackerError::IoError(err)
    }
}

// =================================================================================================================================
// IETF Datatracker API:

pub struct Datatracker {
    connection : reqwest::Client
}

impl Datatracker {
    fn retrieve<T>(&self, url : &str) -> Result<T, DatatrackerError>
        where for<'de> T: Deserialize<'de> 
    {
        let mut res = self.connection.get(url).send()?;
        if res.status().is_success() {
            let res : T = res.json()?;
            Ok(res)
        } else {
            Err(DatatrackerError::NotFound)
        }
    }

    pub fn new() -> Self {
        Datatracker {
            connection : reqwest::Client::new()
        }
    }

    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about email addresses:
    //
    // * https://datatracker.ietf.org/api/v1/person/email/csp@csperkins.org/
    // * https://datatracker.ietf.org/api/v1/person/historicalemail/

    /// Retrieve information about an email address.
    ///
    /// This returns the information held about a particular email address.
    /// If you want information about the person with a particular address,
    /// use `person_from_email()`.
    pub fn email(&self, email : &str) -> Result<Email, DatatrackerError> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/email/{}/", email);
        self.retrieve::<Email>(&url)
    }

    pub fn email_history_for_address<'a>(&'a self, email : &'a str) -> PaginatedList<HistoricalEmail> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalemail/?address={}", email);
        PaginatedList::<'a, HistoricalEmail>::new(self, url)
    }

    pub fn email_history_for_person<'a>(&'a self, person : &'a Person) -> PaginatedList<HistoricalEmail> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalemail/?person={}", person.id);
        PaginatedList::<'a, HistoricalEmail>::new(self, url)
    }

    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about people:
    //
    // * https://datatracker.ietf.org/api/v1/person/person/20209/
    // * https://datatracker.ietf.org/api/v1/person/person/
    // * https://datatracker.ietf.org/api/v1/person/historicalperson/
    //   https://datatracker.ietf.org/api/v1/person/alias/

    pub fn person(&self, person_uri : &PersonUri) -> Result<Person, DatatrackerError> {
        assert!(person_uri.0.starts_with("/api/v1/person/person/"));
        let url = format!("https://datatracker.ietf.org/{}/", person_uri.0);
        self.retrieve::<Person>(&url)
    }

    pub fn person_from_email(&self, email : &str) -> Result<Person, DatatrackerError> {
        let person = self.email(email)?.person;
        self.person(&person)
    }

    pub fn person_history<'a>(&'a self, person : &'a Person) -> PaginatedList<HistoricalPerson> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalperson/?id={}", person.id);
        PaginatedList::<'a, HistoricalPerson>::new(self, url)
    }

    pub fn people<'a>(&'a self) -> PaginatedList<'a, Person> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/");
        PaginatedList::<'a, Person>::new(self, url)
    }

    pub fn people_with_name<'a>(&'a self, name: &'a str) -> PaginatedList<'a, Person> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?name={}", name);
        PaginatedList::<'a, Person>::new(self, url)
    }

    pub fn people_with_name_containing<'a>(&'a self, name_contains: &'a str) -> PaginatedList<'a, Person> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?name__contains={}", name_contains);
        PaginatedList::<'a, Person>::new(self, url)
    }

    pub fn people_between<'a>(&'a self, start: DateTime<Utc>, before: DateTime<Utc>) -> PaginatedList<'a, Person> {
        let s =  start.format("%Y-%m-%dT%H:%M:%S");
        let b = before.format("%Y-%m-%dT%H:%M:%S");
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?time__gte={}&time__lt={}", &s, &b);
        PaginatedList::<'a, Person>::new(self, url)
    }
}

// =================================================================================================================================
// Test suite:

#[cfg(test)]
mod ietfdata_tests {
    use super::*;

    #[test]
    fn test_email() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let e  = dt.email("csp@csperkins.org")?;

        assert_eq!(e.resource_uri, EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()));
        assert_eq!(e.address,      "csp@csperkins.org");
        assert_eq!(e.person,       PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(e.time,         Utc.ymd(1970, 1, 1).and_hms(23, 59, 59));
        assert_eq!(e.origin,       "author: draft-ietf-mmusic-rfc4566bis");
        assert_eq!(e.primary,      true);
        assert_eq!(e.active,       true);

        // Lookup a non-existing address; this should fail
        assert!(dt.email("nobody@example.com").is_err());

        Ok(())
    }

    #[test]
    fn test_email_history_for_address() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let h : Vec<HistoricalEmail> = dt.email_history_for_address("csp@isi.edu").collect();

        assert_eq!(h.len(), 1);
        assert_eq!(h[0].address, "csp@isi.edu");
        assert_eq!(h[0].person,  PersonUri("/api/v1/person/person/20209/".to_string()));

        Ok(())
    }

/*
    #[test]
    fn test_email_history_for_person() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let p  = dt.person_from_email("csp@csperkins.org")?;
        for h in dt.email_history_for_person(&p) {
            println!("{:?}", h);
        }
        Ok(())
    }
*/

    #[test]
    fn test_person() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let p  = dt.person(&PersonUri("/api/v1/person/person/20209/".to_string()))?;

        assert_eq!(p.id,              20209);
        assert_eq!(p.resource_uri,    PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(p.name,            "Colin Perkins");
        assert_eq!(p.name_from_draft, Some("Colin Perkins".to_string()));
        assert_eq!(p.biography,       "Colin Perkins is a Senior Lecturer (Associate Professor) in the School of Computing Science at the University of Glasgow. His research interests are on transport protocols for real-time and interactive multimedia, and on network protocol design, implementation, and specification. He’s been a participant in the IETF and IRTF since 1996, working primarily in the transport area where he co-chairs the RMCAT working group and is a past chair of the AVT and MMUSIC working groups, and in related IRTF research groups. He proposed and co-chaired the first Applied Networking Research Workshop (ANRW), and has been a long-term participant in the Applied Networking Research Prize (ANRP) awarding committee. He received his BEng in Electronic Engineering in 1992, and my PhD in 1996, both from the Department of Electronics at the University of York.");
        assert_eq!(p.ascii,           "Colin Perkins");
        assert_eq!(p.ascii_short,     None);
        assert_eq!(p.time,            Utc.ymd(2012,2,26).and_hms(0,3,54));
        assert_eq!(p.photo,           Some("https://www.ietf.org/lib/dt/media/photo/Colin-Perkins-sm.jpg".to_string()));
        assert_eq!(p.photo_thumb,     Some("https://www.ietf.org/lib/dt/media/photo/Colin-Perkins-sm_PMIAhXi.jpg".to_string()));
        assert_eq!(p.user,            Some("".to_string()));
        assert_eq!(p.consent,         Some(true));
        Ok(())
    }

    #[test]
    fn test_person_from_email() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let p  = dt.person_from_email("csp@csperkins.org")?;

        assert_eq!(p.id,   20209);
        assert_eq!(p.name, "Colin Perkins");

        Ok(())
    }

/*
    #[test]
    fn test_people() {
        let dt = Datatracker::new();
        let people = dt.people();
        for person in people.into_iter() {
            println!("{:?}", person);
        }
    }
*/

    #[test]
    fn test_people_with_name() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let people : Vec<Person> = dt.people_with_name("Colin Perkins").collect();

        assert_eq!(people[0].id,   20209);
        assert_eq!(people[0].name, "Colin Perkins");

        Ok(())
    }

    #[test]
    fn test_people_with_name_containing() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let people : Vec<Person> = dt.people_with_name_containing("Perkins").collect();

        // As of 2019-08-17, there are six people named Perkins in the datatracker.
        assert_eq!(people.len(), 6);

        Ok(())
    }

    #[test]
    fn test_people_between() -> Result<(), DatatrackerError> {
        let start = Utc.ymd(2019, 7, 1).and_hms( 0,  0,  0);
        let until = Utc.ymd(2019, 7, 7).and_hms(23, 59, 59);

        let dt = Datatracker::new();
        let people : Vec<Person> = dt.people_between(start, until).collect();

        // There are 26 people in the tracker with dates in the first week of July 2019
        assert_eq!(people.len(), 26);

        Ok(())
    }

    #[test]
    fn test_person_history() -> Result<(), DatatrackerError> {
        let dt = Datatracker::new();
        let p  = dt.person_from_email("csp@csperkins.org")?;
        let h  : Vec<HistoricalPerson> = dt.person_history(&p).collect();

        // As of 2019-08-18, there are two history items for csp@csperkins.org
        assert_eq!(h.len(), 2);

        Ok(())
    }
}

// =================================================================================================================================
