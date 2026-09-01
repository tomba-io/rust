use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Return the last 1 000 API requests made in the past 3 months.
    ///
    /// See <https://docs.tomba.io/api/account#logs>
    pub fn logs(
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
        self.call("GET", "logs", &params)
    }
}
