use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Check whether a domain is a webmail or disposable provider.
    ///
    /// See <https://docs.tomba.io/api/domain-status>
    pub fn status(&self, domain: &str) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "domain-status", &params)
    }

    /// Auto-complete company names and retrieve logo and domain
    /// information.
    ///
    /// See <https://docs.tomba.io/api/domain-suggestions>
    pub fn autocomplete(
        &self,
        query: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("query".into(), query.into());
        self.call("GET", "domain-suggestions", &params)
    }
}
