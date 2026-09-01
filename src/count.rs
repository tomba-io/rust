use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Returns the total number of email addresses Tomba has for a
    /// domain.
    ///
    /// See <https://docs.tomba.io/api/finder#email-count>
    pub fn count(&self, domain: &str) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "email-count", &params)
    }
}
