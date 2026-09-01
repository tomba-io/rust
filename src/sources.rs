use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Find the web sources where an email address has been found.
    ///
    /// See <hthttps://docs.tomba.io/api/email#email-sources>
    pub fn email_sources(
        &self,
        email: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        self.call("GET", "email-sources", &params)
    }
}
