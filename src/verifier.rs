use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Verify the deliverability of an email address.
    ///
    /// See <https://developer.tomba.io/#email-verifier>
    pub fn email_verifier(
        &self,
        email: &str,
        webhook_url: Option<&str>,
        enrich_mobile: Option<bool>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        if let Some(v) = enrich_mobile {
            params.insert("enrich_mobile".into(), v.to_string());
        }
        self.call("GET", "email-verifier", &params)
    }
}
