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
use super::document::DocumentUri;
use super::document::DocStateUri;

// --------------------------------------------------------------------------------------------------------------------------------
// Types relating to groups:

#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct GroupUri(String);

#[derive(Deserialize, Debug)]
pub struct Group {
    pub id             : u64,
    pub resource_uri   : GroupUri,
    pub acronym        : String,
    pub name           : String,
    pub description    : String,
    pub charter        : DocumentUri,
    pub ad             : Option<PersonUri>,
    #[serde(deserialize_with="deserialize_time")]
    pub time           : DateTime<Utc>,
    #[serde(rename = "type")]
    pub group_type     : GroupTypeUri,
    pub comments       : String,
    pub parent         : GroupUri,
    pub state          : GroupStateUri,
    pub unused_states  : Vec<DocStateUri>,
    pub unused_tags    : Vec<String>,
    pub list_email     : String,
    pub list_subscribe : String,
    pub list_archive   : String
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct GroupTypeUri(String);


#[derive(Deserialize, Debug)]
struct GroupType {
    pub resource_uri : GroupTypeUri,
    pub name         : String,
    pub verbose_name : String,
    pub slug         : String,
    pub desc         : String,
    pub used         : bool,
    pub order        : u64
}


#[derive(Deserialize, Debug, Eq, PartialEq)]
pub struct GroupStateUri(pub String);


#[derive(Deserialize, Debug)]
pub struct GroupState {
    pub resource_uri : GroupStateUri,
    pub desc         : String,
    pub name         : String,
    pub slug         : String,
    pub used         : bool,
    pub order        : u64
}

// --------------------------------------------------------------------------------------------------------------------------------
