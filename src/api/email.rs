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

use chrono::prelude::*;
use serde::Deserialize;

use super::deserialize_time;
use super::person::PersonUri;

// --------------------------------------------------------------------------------------------------------------------------------
// Types relating to email addresses:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct EmailUri(pub String);


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
pub struct HistoricalEmailUri(pub String);


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

// --------------------------------------------------------------------------------------------------------------------------------
