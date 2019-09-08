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
use super::email::EmailUri;
use super::person::PersonUri;
use super::group::GroupUri;

// --------------------------------------------------------------------------------------------------------------------------------
// Types relating to documents:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct DocumentUri(pub String);


#[derive(Deserialize, Debug)]
pub struct Document {
    pub id                 : u64,
    pub resource_uri       : DocumentUri,
    pub name               : String,
    pub title              : String,
    pub pages              : Option<u64>,
    pub words              : Option<u64>,
    #[serde(deserialize_with="deserialize_time")]
    pub time               : DateTime<Utc>,
    pub notify             : String,
    #[serde(deserialize_with="deserialize_time")]
    pub expires            : DateTime<Utc>,
    #[serde(rename = "type")]
    pub doc_type           : String,            // FIXME
    pub rfc                : Option<u64>,
    pub rev                : String,
    #[serde(rename = "abstract")]
    pub doc_abstract       : String,
    pub internal_comments  : String,
    pub order              : u64,
    pub note               : String,
    pub ad                 : Option<PersonUri>,
    pub shepherd           : Option<EmailUri>,
    pub group              : Option<GroupUri>,
    pub stream             : Option<String>,    // FIXME
    pub std_level          : Option<String>,    // FIXME
    pub intended_std_level : Option<String>,    // FIXME
    pub states             : Vec<DocStateUri>,
    pub submissions        : Vec<SubmissionUri>,
    pub tags               : Vec<String>,
    pub uploaded_filename  : String,
    pub external_url       : String
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct SubmissionUri(pub String);


#[derive(Deserialize, Debug)]
pub struct Submission {
    // FIXME
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct DocStateUri(pub String);


#[derive(Deserialize, Debug)]
pub struct DocState {
    pub id           : u64,
    pub resource_uri : DocStateUri,
    pub name         : String,
    pub desc         : String,
    pub slug         : String,
    pub next_states  : Vec<DocStateUri>,
    pub used         : bool,
    pub order        : u64,
    #[serde(rename = "type")]
    pub state_type   : DocStateTypeUri,
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct DocStateTypeUri(pub String);


#[derive(Deserialize, Debug)]
pub struct DocStateType {
    pub resource_uri : DocStateTypeUri,
    pub slug         : String,
    pub label        : String
}

// --------------------------------------------------------------------------------------------------------------------------------
