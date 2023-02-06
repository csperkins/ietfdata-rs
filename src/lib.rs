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

mod api;

pub use api::*;
pub use api::email::*;
pub use api::person::*;
pub use api::group::*;
pub use api::document::*;

use chrono::prelude::*;

use serde::Deserialize;

// =================================================================================================================================
// IETF Datatracker API:

pub struct Datatracker {
    connection : reqwest::Client
}


impl Datatracker {
    fn retrieve<T>(&self, url : &str) -> DTResult<T>
        where for<'de> T: Deserialize<'de> 
    {
        let mut res = self.connection.get(url).send()?;
        if res.status().is_success() {
            Ok(res.json()?)
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
    // * https://datatracker.ietf.org/api/v1/person/email/csp@csperkins.org/
    // * https://datatracker.ietf.org/api/v1/person/historicalemail/

    pub fn email(&self, email_uri: &EmailUri) -> DTResult<Email> {
        let url = format!("https://datatracker.ietf.org{}", email_uri.0);
        self.retrieve::<Email>(&url)
    }

    pub fn email_from_address(&self, email_addr : &str) -> DTResult<Email> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/email/{}/", email_addr);
        self.retrieve::<Email>(&url)
    }


    pub fn email_history_for_address<'a>(&'a self, email_addr : &'a str) -> DTResult<PaginatedList<HistoricalEmail>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalemail/?address={}", email_addr);
        PaginatedList::<'a, HistoricalEmail>::new(&self.connection, url)
    }


    pub fn email_history_for_person<'a>(&'a self, person : &'a Person) -> DTResult<PaginatedList<HistoricalEmail>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalemail/?person={}", person.id);
        PaginatedList::<'a, HistoricalEmail>::new(&self.connection, url)
    }


    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about people:
    // * https://datatracker.ietf.org/api/v1/person/person/20209/
    // * https://datatracker.ietf.org/api/v1/person/person/
    // * https://datatracker.ietf.org/api/v1/person/historicalperson/
    // * https://datatracker.ietf.org/api/v1/person/alias/

    pub fn person(&self, person_uri : &PersonUri) -> DTResult<Person> {
        let url = format!("https://datatracker.ietf.org{}", person_uri.0);
        self.retrieve::<Person>(&url)
    }


    pub fn person_from_email(&self, email : &EmailUri) -> DTResult<Person> {
        let person = self.email(email)?.person;
        self.person(&person)
    }

    pub fn person_from_email_address(&self, email_addr : &str) -> DTResult<Person> {
        let person = self.email_from_address(email_addr)?.person;
        self.person(&person)
    }


    pub fn person_aliases<'a>(&'a self, person : &'a Person) -> DTResult<PaginatedList<PersonAlias>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/alias/?person={}", person.id);
        PaginatedList::<'a, PersonAlias>::new(&self.connection, url)
    }


    pub fn person_history<'a>(&'a self, person : &'a Person) -> DTResult<PaginatedList<HistoricalPerson>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/historicalperson/?id={}", person.id);
        PaginatedList::<'a, HistoricalPerson>::new(&self.connection, url)
    }


    // FIXME: builder pattern for this, and similar functions
    pub fn people<'a>(&'a self) -> DTResult<PaginatedList<'a, Person>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/");
        PaginatedList::<'a, Person>::new(&self.connection, url)
    }


    pub fn people_with_name<'a>(&'a self, name: &'a str) -> DTResult<PaginatedList<'a, Person>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?name={}", name);
        PaginatedList::<'a, Person>::new(&self.connection, url)
    }


    pub fn people_with_name_containing<'a>(&'a self, name_contains: &'a str) -> DTResult<PaginatedList<'a, Person>> {
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?name__contains={}", name_contains);
        PaginatedList::<'a, Person>::new(&self.connection, url)
    }


    pub fn people_between<'a>(&'a self, start: DateTime<Utc>, before: DateTime<Utc>) -> DTResult<PaginatedList<'a, Person>> {
        let s =  start.format("%Y-%m-%dT%H:%M:%S");
        let b = before.format("%Y-%m-%dT%H:%M:%S");
        let url = format!("https://datatracker.ietf.org/api/v1/person/person/?time__gte={}&time__lt={}", &s, &b);
        PaginatedList::<'a, Person>::new(&self.connection, url)
    }


    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about documents:
    //   https://datatracker.ietf.org/api/v1/doc/document/                        - list of documents
    //   https://datatracker.ietf.org/api/v1/doc/document/draft-ietf-avt-rtp-new/ - info about document
    //   https://datatracker.ietf.org/api/v1/doc/docalias/?name=/                 - draft that became the given RFC
    // * https://datatracker.ietf.org/api/v1/doc/state/                           - Types of state a document can be in
    // * https://datatracker.ietf.org/api/v1/doc/statetype/                       - Possible types of state for a document
    //   https://datatracker.ietf.org/api/v1/doc/docevent/                        - list of document events
    //   https://datatracker.ietf.org/api/v1/doc/docevent/?doc=...                - events for a document
    //   https://datatracker.ietf.org/api/v1/doc/docevent/?by=...                 - events by a person (as /api/v1/person/person)
    //   https://datatracker.ietf.org/api/v1/doc/docevent/?time=...               - events by time
    //   https://datatracker.ietf.org/api/v1/doc/documentauthor/?document=...     - authors of a document
    //   https://datatracker.ietf.org/api/v1/doc/documentauthor/?person=...       - documents by person (as /api/v1/person/person)
    //   https://datatracker.ietf.org/api/v1/doc/documentauthor/?email=...        - documents by person with particular email
    //   https://datatracker.ietf.org/api/v1/doc/dochistory/
    //   https://datatracker.ietf.org/api/v1/doc/dochistoryauthor/
    //   https://datatracker.ietf.org/api/v1/doc/docreminder/
    //   https://datatracker.ietf.org/api/v1/doc/documenturl/
    //   https://datatracker.ietf.org/api/v1/doc/statedocevent/                   - subset of /api/v1/doc/docevent/; same parameters
    //   https://datatracker.ietf.org/api/v1/doc/ballotdocevent/                  -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/newrevisiondocevent/             -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/submissiondocevent/              -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/writeupdocevent/                 -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/consensusdocevent/               -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/ballotpositiondocevent/          -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/reviewrequestdocevent/           -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/lastcalldocevent/                -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/telechatdocevent/                -               "                "
    //   https://datatracker.ietf.org/api/v1/doc/relateddocument/?source=...      - documents that source draft relates to (references, replaces, etc)
    //   https://datatracker.ietf.org/api/v1/doc/relateddocument/?target=...      - documents that relate to target draft
    //   https://datatracker.ietf.org/api/v1/doc/ballottype/                      - Types of ballot that can be issued on a document
    //   https://datatracker.ietf.org/api/v1/doc/relateddochistory/
    //   https://datatracker.ietf.org/api/v1/doc/initialreviewdocevent/
    //   https://datatracker.ietf.org/api/v1/doc/deletedevent/
    //   https://datatracker.ietf.org/api/v1/doc/addedmessageevent/
    //   https://datatracker.ietf.org/api/v1/doc/editedauthorsdocevent/

    pub fn doc_state(&self, state_uri: &DocStateUri) -> DTResult<DocState> {
        let url = format!("https://datatracker.ietf.org{}", state_uri.0);
        println!("{:?}", url);
        self.retrieve::<DocState>(&url)
    }


    pub fn doc_states<'a>(&'a self) -> DTResult<PaginatedList<'a, DocState>> {
        let url = format!("https://datatracker.ietf.org/api/v1/doc/state/");
        PaginatedList::<'a, DocState>::new(&self.connection, url)
    }


    pub fn doc_state_type(&self, state_type_uri: &DocStateTypeUri) -> DTResult<DocStateType> {
        let url = format!("https://datatracker.ietf.org{}", state_type_uri.0);
        self.retrieve::<DocStateType>(&url)
    }


    pub fn doc_state_types<'a>(&'a self) -> DTResult<PaginatedList<'a, DocStateType>> {
        let url = format!("https://datatracker.ietf.org/api/v1/doc/statetype/");
        PaginatedList::<'a, DocStateType>::new(&self.connection, url)
    }


    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about names:
    //   https://datatracker.ietf.org/api/v1/name/doctypename/
    //   https://datatracker.ietf.org/api/v1/name/streamname/
    //   https://datatracker.ietf.org/api/v1/name/dbtemplatetypename/
    //   https://datatracker.ietf.org/api/v1/name/docrelationshipname/
    //   https://datatracker.ietf.org/api/v1/name/doctagname/
    //   https://datatracker.ietf.org/api/v1/name/docurltagname/
    //   https://datatracker.ietf.org/api/v1/name/groupstatename/
    //   https://datatracker.ietf.org/api/v1/name/formallanguagename/
    //   https://datatracker.ietf.org/api/v1/name/timeslottypename/
    //   https://datatracker.ietf.org/api/v1/name/liaisonstatementeventtypename/
    //   https://datatracker.ietf.org/api/v1/name/stdlevelname/
    //   https://datatracker.ietf.org/api/v1/name/ballotpositionname/
    //   https://datatracker.ietf.org/api/v1/name/reviewrequeststatename/
    //   https://datatracker.ietf.org/api/v1/name/groupmilestonestatename/
    //   https://datatracker.ietf.org/api/v1/name/iprlicensetypename/
    //   https://datatracker.ietf.org/api/v1/name/feedbacktypename/
    //   https://datatracker.ietf.org/api/v1/name/reviewtypename/
    //   https://datatracker.ietf.org/api/v1/name/iprdisclosurestatename/
    //   https://datatracker.ietf.org/api/v1/name/reviewresultname/
    //   https://datatracker.ietf.org/api/v1/name/liaisonstatementstate/
    //   https://datatracker.ietf.org/api/v1/name/roomresourcename/
    //   https://datatracker.ietf.org/api/v1/name/liaisonstatementtagname/
    //   https://datatracker.ietf.org/api/v1/name/topicaudiencename/
    //   https://datatracker.ietf.org/api/v1/name/continentname/
    //   https://datatracker.ietf.org/api/v1/name/nomineepositionstatename/
    //   https://datatracker.ietf.org/api/v1/name/importantdatename/
    //   https://datatracker.ietf.org/api/v1/name/liaisonstatementpurposename/
    //   https://datatracker.ietf.org/api/v1/name/constraintname/
    //   https://datatracker.ietf.org/api/v1/name/sessionstatusname/
    //   https://datatracker.ietf.org/api/v1/name/ipreventtypename/
    //   https://datatracker.ietf.org/api/v1/name/agendatypename/
    //   https://datatracker.ietf.org/api/v1/name/docremindertypename/
    //   https://datatracker.ietf.org/api/v1/name/intendedstdlevelname/
    //   https://datatracker.ietf.org/api/v1/name/countryname/
    //   https://datatracker.ietf.org/api/v1/name/grouptypename/
    //   https://datatracker.ietf.org/api/v1/name/draftsubmissionstatename/
    //   https://datatracker.ietf.org/api/v1/name/rolename/



    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about working groups:
    //   https://datatracker.ietf.org/api/v1/group/group/                               - list of groups
    //   https://datatracker.ietf.org/api/v1/group/group/2161/                          - info about group 2161
    //   https://datatracker.ietf.org/api/v1/group/grouphistory/?group=2161             - history
    //   https://datatracker.ietf.org/api/v1/group/groupurl/?group=2161                 - URLs
    //   https://datatracker.ietf.org/api/v1/group/groupevent/?group=2161               - events
    //   https://datatracker.ietf.org/api/v1/group/groupmilestone/?group=2161           - Current milestones
    //   https://datatracker.ietf.org/api/v1/group/groupmilestonehistory/?group=2161    - Previous milestones
    //   https://datatracker.ietf.org/api/v1/group/milestonegroupevent/?group=2161      - changed milestones
    //   https://datatracker.ietf.org/api/v1/group/role/?group=2161                     - The current WG chairs and ADs of a group
    //   https://datatracker.ietf.org/api/v1/group/role/?person=20209                   - Groups a person is currently involved with
    //   https://datatracker.ietf.org/api/v1/group/role/?email=csp@csperkins.org        - Groups a person is currently involved with
    //   https://datatracker.ietf.org/api/v1/group/rolehistory/?group=2161              - The previous WG chairs and ADs of a group
    //   https://datatracker.ietf.org/api/v1/group/rolehistory/?person=20209            - Groups person was previously involved with
    //   https://datatracker.ietf.org/api/v1/group/rolehistory/?email=csp@csperkins.org - Groups person was previously involved with
    //   https://datatracker.ietf.org/api/v1/group/changestategroupevent/?group=2161    - Group state changes
    //   https://datatracker.ietf.org/api/v1/group/groupstatetransitions                - ???



    // ----------------------------------------------------------------------------------------------------------------------------
    // Datatracker API endpoints returning information about meetings:
    //   https://datatracker.ietf.org/api/v1/meeting/meeting/                        - list of meetings
    //   https://datatracker.ietf.org/api/v1/meeting/meeting/747/                    - information about meeting number 747
    //   https://datatracker.ietf.org/api/v1/meeting/session/                        - list of all sessions in meetings
    //   https://datatracker.ietf.org/api/v1/meeting/session/25886/                  - a session in a meeting
    //   https://datatracker.ietf.org/api/v1/meeting/session/?meeting=747            - sessions in meeting number 747
    //   https://datatracker.ietf.org/api/v1/meeting/session/?meeting=747&group=2161 - sessions in meeting number 747 for group 2161
    //   https://datatracker.ietf.org/api/v1/meeting/schedtimesessassignment/59003/  - a schededuled session within a meeting
    //   https://datatracker.ietf.org/api/v1/meeting/timeslot/9480/                  - a time slot within a meeting (time, duration, location)
    //   https://datatracker.ietf.org/api/v1/meeting/schedule/791/                   - a draft of the meeting agenda
    //   https://datatracker.ietf.org/api/v1/meeting/room/537/                       - a room at a meeting
    //   https://datatracker.ietf.org/api/v1/meeting/floorplan/14/                   - floor plan for a meeting venue
    //   https://datatracker.ietf.org/api/v1/name/meetingtypename/



}
// =================================================================================================================================
// Test suite:

#[cfg(test)]
mod ietfdata_tests {
    use super::*;

    // ----------------------------------------------------------------------------------------------------------------------------
    // Tests relating to email:

    #[test]
    fn test_email() -> DTResult<()> {
        let dt = Datatracker::new();

        let e  = dt.email(&EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()))?;
        assert_eq!(e.resource_uri, EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()));
        assert_eq!(e.address,      "csp@csperkins.org");
        assert_eq!(e.person,       PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(e.time,         Utc.ymd(1970, 1, 1).and_hms(23, 59, 59));
        assert_eq!(e.primary,      true);
        assert_eq!(e.active,       true);

        Ok(())
    }


    #[test]
    fn test_email_from_address() -> DTResult<()> {
        let dt = Datatracker::new();

        // Lookup an address that exists:
        let e  = dt.email_from_address("csp@csperkins.org")?;
        assert_eq!(e.resource_uri, EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()));
        assert_eq!(e.address,      "csp@csperkins.org");
        assert_eq!(e.person,       PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(e.time,         Utc.ymd(1970, 1, 1).and_hms(23, 59, 59));
        assert_eq!(e.primary,      true);
        assert_eq!(e.active,       true);

        // Lookup a non-existing address; this should fail
        assert!(dt.email_from_address("nobody@example.com").is_err());

        Ok(())
    }


    #[test]
    fn test_email_history_for_address() -> DTResult<()> {
        let dt = Datatracker::new();

        let h  = dt.email_history_for_address("csp@isi.edu")?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(h.len(), 6);
        assert_eq!(h[0].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/167444/".to_string()));
        assert_eq!(h[1].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/161025/".to_string()));
        assert_eq!(h[2].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/128355/".to_string()));
        assert_eq!(h[3].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/128350/".to_string()));
        assert_eq!(h[4].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/71987/".to_string()));
        assert_eq!(h[5].resource_uri, HistoricalEmailUri("/api/v1/person/historicalemail/2090/".to_string()));

        Ok(())
    }

    // ----------------------------------------------------------------------------------------------------------------------------
    // Tests relating to people:

/*
    #[test]
    fn test_email_history_for_person() -> DTResult<()> {
        let dt = Datatracker::new();
        let p  = dt.person_from_email_address("csp@csperkins.org")?;
        for h in dt.email_history_for_person(&p) {
            println!("{:?}", h);
        }
        Ok(())
    }
*/


    #[test]
    fn test_person() -> DTResult<()> {
        let dt = Datatracker::new();

        let p  = dt.person(&PersonUri("/api/v1/person/person/20209/".to_string()))?;
        assert_eq!(p.id,              20209);
        assert_eq!(p.resource_uri,    PersonUri("/api/v1/person/person/20209/".to_string()));
        assert_eq!(p.name,            "Colin Perkins");
        assert_eq!(p.name_from_draft, Some("Colin Perkins".to_string()));
        assert_eq!(p.ascii,           "Colin Perkins");
        assert_eq!(p.ascii_short,     Some("".to_string()));
        assert_eq!(p.time,            Utc.ymd(2012,2,26).and_hms(0,3,54));
        assert_eq!(p.photo,           Some("https://www.ietf.org/lib/dt/media/photo/csp-square.jpg".to_string()));
        assert_eq!(p.photo_thumb,     Some("https://www.ietf.org/lib/dt/media/photo/csp-square_GDMMZmn.jpg".to_string()));
        assert_eq!(p.user,            Some("".to_string()));
        Ok(())
    }

    #[test]
    fn test_person_from_email() -> DTResult<()> {
        let dt = Datatracker::new();

        let p  = dt.person_from_email(&EmailUri("/api/v1/person/email/csp@csperkins.org/".to_string()))?;
        assert_eq!(p.id,   20209);
        assert_eq!(p.name, "Colin Perkins");

        Ok(())
    }



    #[test]
    fn test_person_from_email_address() -> DTResult<()> {
        let dt = Datatracker::new();

        let p  = dt.person_from_email_address("csp@csperkins.org")?;
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
    fn test_people_with_name() -> DTResult<()> {
        let dt = Datatracker::new();

        let people = dt.people_with_name("Colin Perkins")?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(people[0].id,   20209);
        assert_eq!(people[0].name, "Colin Perkins");

        Ok(())
    }


    #[test]
    fn test_people_with_name_containing() -> DTResult<()> {
        let dt = Datatracker::new();

        let people = dt.people_with_name_containing("Perkins")?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(people.len(), 8); // As of 2022-05-02, there are 8 people named Perkins in the datatracker.

        Ok(())
    }


    #[test]
    fn test_people_between() -> DTResult<()> {
        let dt = Datatracker::new();

        let start = Utc.ymd(2019, 7, 1).and_hms( 0,  0,  0);
        let until = Utc.ymd(2019, 7, 7).and_hms(23, 59, 59);
        let people = dt.people_between(start, until)?.collect::<Result<Vec<_>, _>>()?;

        assert_eq!(people.len(), 25); // There are 25 people in the tracker dated in the first week of July 2019

        Ok(())
    }


    #[test]
    fn test_person_history() -> DTResult<()> {
        let dt = Datatracker::new();

        let p  = dt.person_from_email_address("csp@csperkins.org")?;
        println!("{:?}", p);
        let h  = dt.person_history(&p)?.collect::<Result<Vec<_>, _>>()?;
        println!("{:?}", h);
        assert_eq!(h.len(), 8);
        assert_eq!(h[0].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/27668/".to_string()));
        assert_eq!(h[1].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/24980/".to_string()));
        assert_eq!(h[2].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/24978/".to_string()));
        assert_eq!(h[3].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/17735/".to_string()));
        assert_eq!(h[4].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/17734/".to_string()));
        assert_eq!(h[5].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/11731/".to_string()));
        assert_eq!(h[6].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/10878/".to_string()));
        assert_eq!(h[7].resource_uri, HistoricalPersonUri("/api/v1/person/historicalperson/127/".to_string()));

        Ok(())
    }


    #[test]
    fn test_person_aliases() -> DTResult<()> {
        let dt = Datatracker::new();

        let p  = dt.person_from_email_address("csp@csperkins.org")?;
        let h  = dt.person_aliases(&p)?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(h.len(), 2); // As of 2019-08-18, there are two aliases for csp@csperkins.org
        assert_eq!(h[0].name, "Dr. Colin Perkins");
        assert_eq!(h[1].name, "Colin Perkins");

        Ok(())
    }

    // ----------------------------------------------------------------------------------------------------------------------------
    // Tests relating to documents:

    #[test]
    fn test_doc_state() -> DTResult<()> {
        let dt = Datatracker::new();

        let uri = DocStateUri("/api/v1/doc/state/81/".to_string());
        let st  = dt.doc_state(&uri)?;
        assert_eq!(st.id,           81);
        assert_eq!(st.resource_uri, uri);
        assert_eq!(st.name,         "Active");
        assert_eq!(st.desc,         "");
        assert_eq!(st.slug,         "active");
        assert_eq!(st.next_states,  vec!());
        assert_eq!(st.used,         true);
        assert_eq!(st.order,        1);
        assert_eq!(st.state_type,   DocStateTypeUri("/api/v1/doc/statetype/agenda/".to_string()));

        Ok(())
    }

    #[test]
    fn test_doc_states() -> DTResult<()> {
        let dt = Datatracker::new();

        let st = dt.doc_states()?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(st.len(), 171);
        Ok(())
    }

    #[test]
    fn test_doc_state_type() -> DTResult<()> {
        let dt = Datatracker::new();

        let uri = DocStateTypeUri("/api/v1/doc/statetype/draft/".to_string());
        let st  = dt.doc_state_type(&uri)?;
        assert_eq!(st.resource_uri, uri);
        assert_eq!(st.slug,  "draft");
        assert_eq!(st.label, "State");

        Ok(())
    }

    #[test]
    fn test_doc_state_types() -> DTResult<()> {
        let dt = Datatracker::new();

        let st = dt.doc_state_types()?.collect::<Result<Vec<_>, _>>()?;
        assert_eq!(st.len(), 29);
        assert_eq!(st[ 0].slug, "draft");
        assert_eq!(st[ 1].slug, "draft-iesg");
        assert_eq!(st[ 2].slug, "draft-iana");
        assert_eq!(st[ 3].slug, "draft-rfceditor");
        assert_eq!(st[ 4].slug, "draft-stream-ietf");
        assert_eq!(st[ 5].slug, "draft-stream-irtf");
        assert_eq!(st[ 6].slug, "draft-stream-ise");
        assert_eq!(st[ 7].slug, "draft-stream-iab");
        assert_eq!(st[ 8].slug, "slides");
        assert_eq!(st[ 9].slug, "minutes");
        assert_eq!(st[10].slug, "agenda");
        assert_eq!(st[11].slug, "liai-att");
        assert_eq!(st[12].slug, "charter");
        assert_eq!(st[13].slug, "conflrev");
        assert_eq!(st[14].slug, "draft-iana-action");
        assert_eq!(st[15].slug, "draft-iana-review");
        assert_eq!(st[16].slug, "statchg");
        assert_eq!(st[17].slug, "recording");
        assert_eq!(st[18].slug, "bluesheets");
        assert_eq!(st[19].slug, "reuse_policy");
        assert_eq!(st[20].slug, "review");
        assert_eq!(st[21].slug, "liaison");
        assert_eq!(st[22].slug, "shepwrit");
        assert_eq!(st[23].slug, "draft-iana-experts");
        assert_eq!(st[24].slug, "bofreq");
        assert_eq!(st[25].slug, "procmaterials");
        assert_eq!(st[26].slug, "chatlog");
        assert_eq!(st[27].slug, "polls");
        assert_eq!(st[28].slug, "draft-stream-editorial");

        Ok(())
    }
}

// =================================================================================================================================
