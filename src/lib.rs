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

//! # Tomba: Tomba.io is an Email Finder for B2B sales and email marketing
//!
//! This is the Rust client library for the [Email Finder](https://tomba.io) API.
//! It allows you to lookup the following details :
//!
//! - [Domain Search](https://tomba.io/domain-search) are based on the website You give one domain name and it returns all the email addresses found on the internet.
//! - [Email Finder](https://tomba.io/email-finder) generates or retrieves the most likely email address from a domain name, a first name and a last name.
//! - [Email Verifier](https://tomba.io/email-verifier) checks the deliverability of a given email address, verifies if it has been found in our database, and returns their sources.
//! - [Email Enrichment.](https://tomba.io/enrichment) Locate and include data in your emails.
//! - [Author Finder.](https://tomba.io/author-finder) Instantly discover the email addresses of article authors.
//! - [LinkedIn Finder.](https://tomba.io/linkedin-finder) Instantly discover the email addresses of Linkedin URLs.
//!
//! ## Features
//!
//! * Collect publicly available emails online (Html, execute JavaScript,files,).
//! * No duplicate email    No duplicate domain .
//! * No webmail like Gmail,Outlook and the others.
//! * We detect 15 type of hashes and remove them.
//! * No disposable and temporary email address.
//! ## Example
//!
//! ```no_run
//! use tomba::{Tomba, TombaConfig};
//! 
//! let config = TombaConfig {
//!    key: "ta_xxxx".to_string(),
//!    secret: "ts_xxxx".to_string(),
//! };
//! let mut tomba = Tomba::init(config).expect("should construct");
//! 
//! let res = tomba.account();
//! ```
//! 

/// DEFAULT BASE URL
const DEFAULT_BASE_URL: &str = &"https://api.tomba.io/v1/";

/// Account path
const ACCOUNT_PATH: &str = "me";

/// Usage path
const USAGE_PATH: &str = "usage";

/// Logs path
const LOGS_PATH: &str = "logs";

/// Search path
const SEARCH_PATH: &str = "domain-search/";

/// Finder path
const FINDER_PATH: &str = "email-finder/";

/// Verifier path
const VERIFIER_PATH: &str = "email-verifier/";

/// Email Sources path
const SOURCES_PATH: &str = "email-sources/";

/// Email Count path
const COUNT_PATH: &str = "email-count/";

/// Domain status path
const STATUS_PATH: &str = "domain-status/";

/// Domain status path
const AUTOCOMPLETE_PATH: &str = "domains-suggestion/";

#[macro_use]
mod tomba;
mod account;
mod autocomplete;
// mod error;
mod count;
mod finder;
mod logs;
mod search;
mod sources;
mod status;
mod usage;
mod verifier;

pub use crate::tomba::*;
pub use account::*;
pub use autocomplete::*;
// pub use error::*;
pub use count::*;
pub use finder::*;
pub use logs::*;
pub use search::*;
pub use sources::*;
pub use status::*;
pub use usage::*;
pub use verifier::*;
