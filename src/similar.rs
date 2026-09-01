use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Find domains similar to the given one.
    ///
    /// See <https://docs.tomba.io/api/domain#similar>
    pub fn similar_domains(
        &self,
        domain: &str,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        self.call("GET", "similar", &params)
    }
}
