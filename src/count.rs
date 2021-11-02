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

//! Tomba Email Count data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Count {
    #[serde(rename = "data")]
    pub data: CountData,
}

#[derive(Serialize, Deserialize)]
pub struct CountData {
    #[serde(rename = "total")]
    pub total: i64,

    #[serde(rename = "personal_emails")]
    pub personal_emails: i64,

    #[serde(rename = "generic_emails")]
    pub generic_emails: i64,

    #[serde(rename = "department")]
    pub department: Department,

    #[serde(rename = "seniority")]
    pub seniority: Seniority,
}

#[derive(Serialize, Deserialize)]
pub struct Department {
    #[serde(rename = "engineering")]
    pub engineering: i64,

    #[serde(rename = "finance")]
    pub finance: i64,

    #[serde(rename = "hr")]
    pub hr: i64,

    #[serde(rename = "it")]
    pub it: i64,

    #[serde(rename = "marketing")]
    pub marketing: i64,

    #[serde(rename = "operations")]
    pub operations: i64,

    #[serde(rename = "management")]
    pub management: i64,

    #[serde(rename = "sales")]
    pub sales: i64,

    #[serde(rename = "legal")]
    pub legal: i64,

    #[serde(rename = "support")]
    pub support: i64,

    #[serde(rename = "communication")]
    pub communication: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Seniority {
    #[serde(rename = "junior")]
    pub junior: i64,

    #[serde(rename = "senior")]
    pub senior: i64,

    #[serde(rename = "executive")]
    pub executive: i64,
}
