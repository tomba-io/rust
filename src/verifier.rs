//   Copyright 2021 Tomba technology web service LLC
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

//! Email Verifier data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Verifier {
    #[serde(rename = "data")]
    pub data: VerifierData,
}

#[derive(Serialize, Deserialize)]
pub struct VerifierData {
    #[serde(rename = "email")]
    pub email: VerifierEmail,

    #[serde(rename = "sources")]
    pub sources: Vec<VerifierSource>,
}

#[derive(Serialize, Deserialize)]
pub struct VerifierEmail {
    #[serde(rename = "mx_records")]
    pub mx_records: bool,

    #[serde(rename = "smtp_server")]
    pub smtp_server: Option<serde_json::Value>,

    #[serde(rename = "smtp_check")]
    pub smtp_check: bool,

    #[serde(rename = "accept_all")]
    pub accept_all: Option<serde_json::Value>,

    #[serde(rename = "block")]
    pub block: Option<serde_json::Value>,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "gibberish")]
    pub gibberish: bool,

    #[serde(rename = "disposable")]
    pub disposable: bool,

    #[serde(rename = "webmail")]
    pub webmail: bool,

    #[serde(rename = "regex")]
    pub regex: bool,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "result")]
    pub result: String,

    #[serde(rename = "score")]
    pub score: i64,
}

#[derive(Serialize, Deserialize)]
pub struct VerifierSource {
    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "extracted_on")]
    pub extracted_on: String,

    #[serde(rename = "last_seen_on")]
    pub last_seen_on: String,

    #[serde(rename = "still_on_page")]
    pub still_on_page: bool,

    #[serde(rename = "website_url")]
    pub website_url: String,
}
