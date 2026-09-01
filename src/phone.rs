use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Find a phone number using an email address.
    ///
    /// See <https://docs.tomba.io/api/phone#phone-finder>
    pub fn phone_finder(
        &self,
        email: &str,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "phone-finder", &params)
    }

    /// Validate a phone number.
    ///
    /// See <https://docs.tomba.io/api/phone#phone-validator>
    pub fn phone_validator(
        &self,
        phone: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("phone".into(), phone.into());
        self.call("GET", "phone-validator", &params)
    }
}
