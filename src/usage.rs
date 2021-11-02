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

//! Tomba Usage data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Usage {
    #[serde(rename = "data")]
    pub data: Vec<UsageTotal>,

    #[serde(rename = "total")]
    pub total: UsageTotal,
}

#[derive(Serialize, Deserialize)]
pub struct UsageTotal {
    #[serde(rename = "usage")]
    pub usage: i64,

    #[serde(rename = "created_at")]
    pub created_at: Option<String>,

    #[serde(rename = "domain")]
    pub domain: i64,

    #[serde(rename = "finder")]
    pub finder: i64,

    #[serde(rename = "verifier")]
    pub verifier: i64,

    #[serde(rename = "technologies")]
    pub technologies: i64,

    #[serde(rename = "ip")]
    pub ip: Option<i64>,

    #[serde(rename = "website")]
    pub website: i64,

    #[serde(rename = "bulk")]
    pub bulk: i64,

    #[serde(rename = "extension")]
    pub extension: i64,

    #[serde(rename = "api")]
    pub api: i64,

    #[serde(rename = "mobile")]
    pub mobile: i64,

    #[serde(rename = "desktop")]
    pub desktop: i64,

    #[serde(rename = "sheets")]
    pub sheets: i64,

    #[serde(rename = "socail")]
    pub socail: Option<i64>,
}
