use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Check what technologies a website uses.
    ///
    /// See <https://docs.tomba.io/api/~endpoints#technology>
    pub fn technology_check(
        &self,
        domain: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "technology-checker", &params)
    }
}
