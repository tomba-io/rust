use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all lead lists.
    ///
    /// See <https://docs.tomba.io/api/leads-lists>
    pub fn list_leads_lists(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "leads_lists", &HashMap::new())
    }

    /// Get a single leads list by ID.
    ///
    /// See <https://docs.tomba.io/api/leads#get-leads-list>
    pub fn get_leads_list(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_lists/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new leads list.
    ///
    /// See <https://docs.tomba.io/api/leads#create-leads-list>
    pub fn create_leads_list(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "leads_lists", body)
    }

    /// Update an existing leads list.
    ///
    /// See <https://docs.tomba.io/api/leads#update-leads-list>
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
    /// See <https://docs.tomba.io/api/leads#delete-leads-list>
    pub fn delete_leads_list(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_lists/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }
}
