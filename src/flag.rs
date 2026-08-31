use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all flags.
    ///
    /// See <https://developer.tomba.io/#list-all-flags>
    pub fn list_flags(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        if let Some(v) = page {
            params.insert("page".into(), v.to_string());
        }
        if let Some(v) = limit {
            params.insert("limit".into(), v.to_string());
        }
        self.call("GET", "flags", &params)
    }

    /// Create a new flag.
    ///
    /// See <https://developer.tomba.io/#create-a-flag>
    pub fn create_flag(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "flags", body)
    }
}
