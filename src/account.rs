use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Returns information about the current account.
    ///
    /// See <https://docs.tomba.io/api/account>
    pub fn account(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "me", &HashMap::new())
    }
}
