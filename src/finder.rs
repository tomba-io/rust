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

//! Tomba Email Finder data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Finder {
    #[serde(rename = "data")]
    pub data: FinderData,
}

#[derive(Serialize, Deserialize)]
pub struct FinderData {
    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "full_name")]
    pub full_name: String,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "position")]
    pub position: Option<serde_json::Value>,

    #[serde(rename = "twitter")]
    pub twitter: Option<serde_json::Value>,

    #[serde(rename = "linkedin")]
    pub linkedin: String,

    #[serde(rename = "phone_number")]
    pub phone_number: Option<serde_json::Value>,

    #[serde(rename = "accept_all")]
    pub accept_all: Option<serde_json::Value>,

    #[serde(rename = "website_url")]
    pub website_url: String,

    #[serde(rename = "company")]
    pub company: String,

    #[serde(rename = "score")]
    pub score: i64,

    #[serde(rename = "verification")]
    pub verification: FinderVerification,

    #[serde(rename = "sources")]
    pub sources: Vec<Option<serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct FinderVerification {
    #[serde(rename = "date")]
    pub date: Option<serde_json::Value>,

    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,
}
