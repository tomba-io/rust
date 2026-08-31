use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all lead lists.
    ///
    /// See <https://developer.tomba.io/#list-all-leads-lists>
    pub fn list_leads_lists(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "leads_lists", &HashMap::new())
    }

    /// Get a single leads list by ID.
    ///
    /// See <https://developer.tomba.io/#get-a-leads-list>
    pub fn get_leads_list(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_lists/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new leads list.
    ///
    /// See <https://developer.tomba.io/#create-a-leads-list>
    pub fn create_leads_list(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "leads_lists", body)
    }

    /// Update an existing leads list.
    ///
    /// See <https://developer.tomba.io/#update-a-leads-list>
    pub fn update_leads_list(
        &self,
        id: &str,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_lists/{}", id);
        self.call_json("PUT", &path, body)
    }

    /// Delete a leads list by ID.
    ///
    /// See <https://developer.tomba.io/#delete-a-leads-list>
    pub fn delete_leads_list(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_lists/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }
}
