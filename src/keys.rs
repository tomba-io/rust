use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all API keys.
    ///
    /// See <https://docs.tomba.io/api/keys>
    pub fn list_keys(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "keys", &HashMap::new())
    }

    /// Get a single API key by its ID.
    ///
    /// See <https://docs.tomba.io/api/keys#get-key>
    pub fn get_key(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("keys/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new API key.
    ///
    /// See <https://docs.tomba.io/api/keys#create-key>
    pub fn create_key(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "keys", body)
    }

    /// Delete an API key.
    ///
    /// See <https://docs.tomba.io/api/keys#delete-key>
    pub fn delete_key(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("keys/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }

    /// Reset an API key.
    ///
    /// See <https://docs.tomba.io/api/keys#reset-key>
    pub fn reset_key(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("keys/{}/reset", id);
        self.call_json("PUT", &path, &serde_json::json!({}))
    }
}
