// Copyright (C) 2019-2020 University of Glasgow
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

use chrono::prelude::*;
use serde::Deserialize;

use super::*;

// --------------------------------------------------------------------------------------------------------------------------------
// Types relating to people:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PersonUri(pub String);


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct PersonAliasUri(pub String);


#[derive(Deserialize, Debug)]
pub struct PersonAlias {
    pub id           : u64,
    pub resource_uri : PersonAliasUri,
    pub person       : PersonUri,
    pub name         : String,
}

// --------------------------------------------------------------------------------------------------------------------------------


pub struct PersonFilter {
    query_url   : String,
    params      : Vec<String>
}

impl PersonFilter {
    fn new(query_url : String) -> PersonFilter {
        PersonFilter {
            query_url : query_url,
            params    : Vec::new()
        }
    }

    fn since(mut self, date : DateTime<Utc>) -> PersonFilter {
        unimplemented!();
    }

    fn until(mut self, date : DateTime<Utc>) -> PersonFilter {
        unimplemented!();
    }

    fn with_name(mut self, name : String) -> PersonFilter {
        self.params.push(format!("name={}", name));
        self
    }

    fn with_name_containing(mut self, name : String) -> PersonFilter {
        self.params.push(format!("name__contains={}", name));
        self
    }

    fn fetch<'a>(self) -> DTResult<PaginatedList<'a, Person>> {
        unimplemented!();
    }
}

// --------------------------------------------------------------------------------------------------------------------------------
