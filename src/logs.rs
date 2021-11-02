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

//! Tomba Logs data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Logs {
    #[serde(rename = "data")]
    pub data: Vec<LogsData>,
}

#[derive(Serialize, Deserialize)]
pub struct LogsData {
    #[serde(rename = "type")]
    pub datum_type: String,

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "cost")]
    pub cost: bool,

    #[serde(rename = "ip_address")]
    pub ip_address: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "source")]
    pub source: String,

    #[serde(rename = "http_method")]
    pub http_method: String,

    #[serde(rename = "country")]
    pub country: String,
}
