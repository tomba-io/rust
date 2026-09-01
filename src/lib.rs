// Copyright 2021 Tomba technology web service LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Tomba -- Email Finder for B2B sales and email marketing
//!
//! This is the official Rust client library for the
//! [Tomba](https://tomba.io) API.
//!
//! ## Supported endpoints
//!
//! | Endpoint | Method |
//! |---|---|
//! | [Domain Search](https://tomba.io/domain-search) | `domain_search` |
//! | [Email Finder](https://tomba.io/email-finder) | `email_finder` |
//! | [Email Verifier](https://tomba.io/email-verifier) | `email_verifier` |
//! | [Email Sources](https://tomba.io/email-sources) | `email_sources` |
//! | [Email Count](https://tomba.io/email-count) | `count` |
//! | [Domain Status](https://tomba.io/domain-status) | `status` |
//! | [Autocomplete](https://tomba.io/autocomplete) | `autocomplete` |
//! | [Phone Finder](https://tomba.io/phone-finder) | `phone_finder` |
//! | [Phone Validator](https://tomba.io/phone-validator) | `phone_validator` |
//! | [Similar Domains](https://tomba.io/similar) | `similar_domains` |
//! | [Technology Checker](https://tomba.io/technology-checker) | `technology_check` |
//! | [Email Format](https://tomba.io/email-format) | `email_format` |
//! | [Location](https://tomba.io/location) | `get_location` |
//! | [Enrichment](https://tomba.io/enrichment) | `person_find`, `company_find`, `combined_find` |
//! | [Companies Search](https://tomba.io/companies-search) | `companies_search` |
//! | [Keys](https://docs.tomba.io/api/keys) | `list_keys`, `get_key`, `create_key`, `delete_key`, `reset_key` |
//! | [Flags](https://docs.tomba.io/api/flag) | `list_flags`, `create_flag` |
//! | [Leads](https://docs.tomba.io/api/leads) | `list_leads`, `get_lead`, `create_lead`, `update_lead`, `delete_lead` |
//! | [Leads Lists](https://docs.tomba.io/api/lead-lists) | `list_leads_lists`, `get_leads_list`, `create_leads_list`, `update_leads_list`, `delete_leads_list` |
//! | [Leads Attributes](https://docs.tomba.io/api/lead-attributes) | `list_leads_attributes`, `get_leads_attribute`, `create_leads_attribute`, `update_leads_attribute`, `delete_leads_attribute` |
//! | [Bulk](https://docs.tomba.io/api/bulks) | `list_bulk`, `get_bulk`, `create_bulk`, `launch_bulk`, `delete_bulk`, `archive_bulk`, `rename_bulk`, `bulk_progress`, `bulk_download` |
//!
//! ## Quick start
//!
//! ```no_run
//! use tomba::{Tomba, TombaConfig};
//!
//! let config = TombaConfig {
//!     key: "ta_xxxx".to_string(),
//!     secret: "ts_xxxx".to_string(),
//! };
//! let tomba = Tomba::init(config).expect("should construct");
//!
//! let response = tomba.account().expect("should get account");
//! println!("{:?}", response.data);
//! println!("{:?}", response.rate_limit);
//! ```

/// Default base URL for the Tomba API.
const DEFAULT_BASE_URL: &str = "https://api.tomba.io/v1/";

mod account;
mod bulk;
mod count;
mod domain;
mod enrichment;
pub mod error;
mod finder;
mod flag;
mod format;
mod keys;
mod leads;
mod leads_attributes;
mod leads_lists;
mod location;
mod logs;
mod phone;
mod reveal;
mod similar;
mod sources;
mod status;
mod technology;
mod tomba;
mod usage;
mod verifier;

pub use crate::tomba::{
    parse_rate_limit, RateLimit, Tomba, TombaConfig, TombaResponse,
};
pub use error::TombaError;
