use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Enrich a person by email address.
    ///
    /// See <https://developer.tomba.io/#enrichment>
    pub fn person_find(
        &self,
        email: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        self.call("GET", "enrichment", &params)
    }

    /// Enrich a company by domain.
    ///
    /// See <https://developer.tomba.io/#company-enrichment>
    pub fn company_find(
        &self,
        domain: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "company-enrichment", &params)
    }

    /// Combined person + company enrichment.
    ///
    /// See <https://developer.tomba.io/#combined-enrichment>
    pub fn combined_find(
        &self,
        email: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        self.call("GET", "combined-enrichment", &params)
    }
}
