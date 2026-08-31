use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Search for companies matching the given criteria.
    ///
    /// See <https://developer.tomba.io/#companies-search>
    pub fn companies_search(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "companies-search", body)
    }
}
