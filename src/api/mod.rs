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

pub mod email;
pub mod person;
pub mod group;
pub mod document;

use std::error;
use std::fmt;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

// =================================================================================================

pub fn deserialize_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&s, "%+").map_err(serde::de::Error::custom)
}

// =================================================================================================
// Generic types representing a paginated list of responses from the Datatracker:

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub total_count : u32,
    pub limit       : u32,
    pub offset      : u32,
    pub previous    : Option<String>,
    pub next        : Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Page<T> {
    pub meta        : Meta,
    pub objects     : Vec<T>
}

pub struct PaginatedList<'a, T> {
    pub iter : <Vec<T> as IntoIterator>::IntoIter,
    pub next : Option<String>,
    pub conn : &'a reqwest::Client
}

impl<'a, T> PaginatedList<'a, T>
    where for<'de> T: Deserialize<'de>
{
    pub fn new(conn: &'a reqwest::Client, url : String) -> Result<Self, DatatrackerError> {
        let mut res = conn.get(&url).send()?;
        let pl : Page<T> = res.json()?;

        Ok(Self {
            next : pl.meta.next.clone(),
            iter : pl.objects.into_iter(),
            conn : conn
        })
    }

    fn try_next(&mut self) -> Result<Option<T>, DatatrackerError> {
        match self.iter.next() {
            Some(x) => {
                Ok(Some(x))
            }
            None => {
                match self.next.clone() {
                    Some(ref url_frag) => {
                        let url = format!("https://datatracker.ietf.org{}", url_frag);
                        let mut res = self.conn.get(&url).send()?;
                        let pl : Page<T> = res.json()?;
                        self.next = pl.meta.next.clone();
                        self.iter = pl.objects.into_iter();
                        self.try_next()
                    }
                    None => {
                        Ok(None)
                    }
                }
            }
        }
    }
}

impl<'a, T> Iterator for PaginatedList<'a, T>
    where for<'de> T: Deserialize<'de>
{
    type Item = Result<T, DatatrackerError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(None)    => None,
            Ok(Some(x)) => Some(Ok(x)),
            Err(e)      => Some(Err(e))
        }
    }
}


// =================================================================================================
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

pub type DTResult<T> = Result<T, DatatrackerError>;

// =================================================================================================
