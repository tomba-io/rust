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

//! Tomba Account data structures.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "data")]
    pub data: AccountData,
}

#[derive(Serialize, Deserialize)]
pub struct AccountData {
    #[serde(rename = "available")]
    pub available: i64,

    #[serde(rename = "user_id")]
    pub user_id: i64,

    #[serde(rename = "secret_token")]
    pub secret_token: String,

    #[serde(rename = "role")]
    pub role: i64,

    #[serde(rename = "confirmed")]
    pub confirmed: bool,

    #[serde(rename = "blocked")]
    pub blocked: bool,

    #[serde(rename = "first_name")]
    pub first_name: String,

    #[serde(rename = "last_name")]
    pub last_name: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "phone")]
    pub phone: String,

    #[serde(rename = "image")]
    pub image: String,

    #[serde(rename = "deleted")]
    pub deleted: bool,

    #[serde(rename = "timezone")]
    pub timezone: String,

    #[serde(rename = "activity")]
    pub activity: bool,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "country")]
    pub country: String,

    #[serde(rename = "created_at")]
    pub created_at: String,

    #[serde(rename = "ip")]
    pub ip: String,

    #[serde(rename = "payment_status")]
    pub payment_status: bool,

    #[serde(rename = "expired")]
    pub expired: String,

    #[serde(rename = "pricing")]
    pub pricing: AccountPricing,

    #[serde(rename = "time")]
    pub time: AccountTime,

    #[serde(rename = "used")]
    pub used: i64,

    #[serde(rename = "requests")]
    pub requests: AccountRequests,
}

#[derive(Serialize, Deserialize)]
pub struct AccountPricing {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "pricing_id")]
    pub pricing_id: i64,

    #[serde(rename = "available_searches")]
    pub available_searches: i64,

    #[serde(rename = "available_verifications")]
    pub available_verifications: i64,

    #[serde(rename = "available_phones")]
    pub available_phones: i64,

    #[serde(rename = "available_leads")]
    pub available_leads: i64,

    #[serde(rename = "available_list")]
    pub available_list: i64,

    #[serde(rename = "available_attr")]
    pub available_attr: i64,

    #[serde(rename = "available_keys")]
    pub available_keys: i64,

    #[serde(rename = "available_teams")]
    pub available_teams: i64,

    #[serde(rename = "available_email_count")]
    pub available_email_count: i64,

    #[serde(rename = "frequency")]
    pub frequency: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountRequests {
    #[serde(rename = "domains")]
    pub domains: AccountDomains,

    #[serde(rename = "verifications")]
    pub verifications: AccountDomains,

    #[serde(rename = "phones")]
    pub phones: AccountDomains,
}

#[derive(Serialize, Deserialize)]
pub struct AccountDomains {
    #[serde(rename = "available")]
    pub available: i64,

    #[serde(rename = "used")]
    pub used: i64,
}

#[derive(Serialize, Deserialize)]
pub struct AccountTime {
    #[serde(rename = "date")]
    pub date: String,

    #[serde(rename = "timezone_type")]
    pub timezone_type: i64,

    #[serde(rename = "timezone")]
    pub timezone: String,
}
