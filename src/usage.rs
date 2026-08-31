use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Return your monthly request usage.
    ///
    /// See <https://developer.tomba.io/#usage>
    pub fn usage(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "usage", &HashMap::new())
    }
}
