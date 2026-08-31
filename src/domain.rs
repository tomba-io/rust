use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Search emails for a domain.
    ///
    /// Returns all email addresses found on the internet for the
    /// given domain.
    ///
    /// See <https://developer.tomba.io/#domain-search>
    pub fn domain_search(
        &self,
        domain: &str,
        enrich_mobile: Option<bool>,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        if let Some(v) = enrich_mobile {
            params.insert("enrich_mobile".into(), v.to_string());
        }
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "domain-search", &params)
    }
}
