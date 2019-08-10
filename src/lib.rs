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

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

// ================================================================================================
// Helper functions:

fn deserialize_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, "%Y-%m-%dT%H:%M:%S").map_err(serde::de::Error::custom)
}

fn deserialize_email_uri<'de, D>(deserializer: D) -> Result<EmailUri, D::Error>
    where D: Deserializer<'de>
{
    Ok(EmailUri(String::deserialize(deserializer)?))
}

fn deserialize_person_uri<'de, D>(deserializer: D) -> Result<PersonUri, D::Error>
    where D: Deserializer<'de>
{
    Ok(PersonUri(String::deserialize(deserializer)?))
}

// ================================================================================================
// IETF Datatracker types:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct EmailUri(String);

/// A mapping from email address to person in the IETF datatracker.
#[derive(Deserialize, Debug)]
pub struct Email {
    #[serde(deserialize_with="deserialize_email_uri")]
    pub resource_uri : EmailUri,
    pub address      : String,
    #[serde(deserialize_with="deserialize_person_uri")]
    pub person       : PersonUri,
    #[serde(deserialize_with="deserialize_time")]
    pub time         : DateTime<Utc>,
    pub origin       : String,
    pub primary      : bool,
    pub active       : bool
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PersonUri(String);

/// A person in the IETF datatracker.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Person {
    pub id              : u64,
    #[serde(deserialize_with="deserialize_person_uri")]
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

// ================================================================================================
// IETF Datatracker API:

pub struct Datatracker {
    connection : reqwest::Client
}

impl Datatracker {
    pub fn new() -> Self {
        Datatracker {
            connection : reqwest::Client::new()
        }
    }

    /// Look-up a person by email address.
    pub fn email(&self, email : &str) -> Email {
        let url = format!("https://datatracker.ietf.org/api/v1/person/email/{}/", email);
        let mut res = self.connection.get(&url).send().unwrap();
        let email : Email = res.json().unwrap();
        email
    }

    pub fn person(&self, person_uri : &PersonUri) -> Person {
        let url = format!("https://datatracker.ietf.org/{}/", person_uri.0);
        let mut res = self.connection.get(&url).send().unwrap();
        let person : Person = res.json().unwrap();
        person
    }
}

// ================================================================================================
// Test suite:

#[cfg(test)]
mod ietfdata_tests {
    use super::*;

    #[test]
    fn test_email() {
        let dt = Datatracker::new();
        let e  = dt.email("csp@csperkins.org");
        assert_eq!(e.resource_uri, EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()));
        assert_eq!(e.address,      "csp@csperkins.org");
        assert_eq!(e.person,       PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(e.time,         Utc.ymd(1970, 1, 1).and_hms(23, 59, 59));
        assert_eq!(e.origin,       "author: draft-ietf-mmusic-rfc4566bis");
        assert_eq!(e.primary,      true);
        assert_eq!(e.active,       true);
    }

    #[test]
    fn test_person() {
        let dt = Datatracker::new();
        let p  = dt.person(&PersonUri("/api/v1/person/person/20209/".to_string()));
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
    }
}

// ================================================================================================
