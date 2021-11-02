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

//! Tomba Domain Search data structures.

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Search {
    #[serde(rename = "data")]
    pub data: SearchData,

    #[serde(rename = "meta")]
    pub meta: SearchMeta,
}

#[derive(Serialize, Deserialize)]
pub struct SearchData {
    #[serde(rename = "organization")]
    pub organization: SearchOrganization,

    #[serde(rename = "emails")]
    pub emails: Vec<SearchEmail>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchEmail {
    #[serde(rename = "email")]
    pub email: Option<String>,

    #[serde(rename = "first_name")]
    pub first_name: Option<String>,

    #[serde(rename = "last_name")]
    pub last_name: Option<String>,

    #[serde(rename = "full_name")]
    pub full_name: Option<String>,

    #[serde(rename = "phone_number")]
    pub phone_number: Option<bool>,

    #[serde(rename = "type")]
    pub email_type: Option<String>,

    #[serde(rename = "country")]
    pub country: Option<String>,

    #[serde(rename = "position")]
    pub position: Option<String>,

    #[serde(rename = "department")]
    pub department: Option<String>,

    #[serde(rename = "seniority")]
    pub seniority: Option<String>,

    #[serde(rename = "twitter")]
    pub twitter: Option<String>,

    #[serde(rename = "linkedin")]
    pub linkedin: Option<String>,

    #[serde(rename = "accept_all")]
    pub accept_all: Option<bool>,

    #[serde(rename = "pattern")]
    pub pattern: Option<String>,

    #[serde(rename = "score")]
    pub score: Option<i64>,

    #[serde(rename = "verification")]
    pub verification: SearchVerification,

    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,

    #[serde(rename = "sources")]
    pub sources: Vec<SearchSource>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchSource {
    #[serde(rename = "uri")]
    pub uri: Option<String>,

    #[serde(rename = "extracted_on")]
    pub extracted_on: Option<String>,

    #[serde(rename = "last_seen_on")]
    pub last_seen_on: Option<String>,

    #[serde(rename = "still_on_page")]
    pub still_on_page: Option<bool>,

    #[serde(rename = "website_url")]
    pub website_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchVerification {
    #[serde(rename = "date")]
    pub date: Option<String>,

    #[serde(rename = "status")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchOrganization {
    #[serde(rename = "location")]
    pub location: SearchLocation,

    #[serde(rename = "social_links")]
    pub social_links: SearchSocialLinks,

    #[serde(rename = "disposable")]
    pub disposable: Option<bool>,

    #[serde(rename = "webmail")]
    pub webmail: Option<bool>,

    #[serde(rename = "website_url")]
    pub website_url: Option<String>,

    #[serde(rename = "phone_number")]
    pub phone_number: Option<String>,

    #[serde(rename = "industries")]
    pub industries: Option<String>,

    #[serde(rename = "postal_code")]
    pub postal_code: Option<String>,

    #[serde(rename = "employee_count")]
    pub employee_count: Option<i64>,

    #[serde(rename = "last_updated")]
    pub last_updated: Option<String>,

    #[serde(rename = "revenue")]
    pub revenue: Option<String>,

    #[serde(rename = "accept_all")]
    pub accept_all: Option<bool>,

    #[serde(rename = "pattern")]
    pub pattern: Option<String>,

    #[serde(rename = "domain_score")]
    pub domain_score: Option<i64>,

    #[serde(rename = "organization")]
    pub organization: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchLocation {
    #[serde(rename = "country")]
    pub country: Option<String>,

    #[serde(rename = "city")]
    pub city: Option<String>,

    #[serde(rename = "state")]
    pub state: Option<String>,

    #[serde(rename = "street_address")]
    pub street_address: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchSocialLinks {
    #[serde(rename = "twitter_url")]
    pub twitter_url: Option<String>,

    #[serde(rename = "facebook_url")]
    pub facebook_url: Option<String>,

    #[serde(rename = "linkedin_url")]
    pub linkedin_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchMeta {
    #[serde(rename = "total")]
    pub total: Option<i64>,

    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,

    #[serde(rename = "current")]
    pub current: Option<i64>,

    #[serde(rename = "total_pages")]
    pub total_pages: Option<i64>,
}