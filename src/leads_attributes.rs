use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all leads attributes.
    ///
    /// See <https://docs.tomba.io/api/leads-attributes>
    pub fn list_leads_attributes(&self) -> Result<TombaResponse, TombaError> {
        self.call("GET", "leads_attributes", &HashMap::new())
    }

    /// Get a single leads attribute by ID.
    ///
    /// See <https://docs.tomba.io/api/leads#get-leads-attribute>
    pub fn get_leads_attribute(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_attributes/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new leads attribute.
    ///
    /// See <https://docs.tomba.io/api/leads#create-leads-attribute>
    pub fn create_leads_attribute(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "leads_attributes", body)
    }

    /// Update an existing leads attribute.
    ///
    /// See <https://docs.tomba.io/api/leads#update-leads-attribute>
    pub fn update_leads_attribute(
        &self,
        id: &str,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_attributes/{}", id);
        self.call_json("PUT", &path, body)
    }

    /// Delete a leads attribute by ID.
    ///
    /// See <https://docs.tomba.io/api/leads#delete-leads-attribute>
    pub fn delete_leads_attribute(
        &self,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads_attributes/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }
}
