use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Get location details for a country code.
    ///
    /// See <https://docs.tomba.io/api/finder#location>
    pub fn get_location(
        &self,
        country: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("country".into(), country.into());
        self.call("GET", "location", &params)
    }
}
