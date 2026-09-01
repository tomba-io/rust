use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Detect the email format used by a company.
    ///
    /// See <https://docs.tomba.io/api/~endpoints#email-format>
    pub fn email_format(
        &self,
        domain: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "email-format", &params)
    }
}
