use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all saved leads.
    ///
    /// See <https://developer.tomba.io/#list-all-leads>
    pub fn list_leads(
        &self,
        domain: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        if let Some(v) = domain {
            params.insert("domain".into(), v.into());
        }
        self.call("GET", "leads", &params)
    }

    /// Get a single lead by ID.
    ///
    /// See <https://developer.tomba.io/#get-a-lead>
    pub fn get_lead(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("leads/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new lead.
    ///
    /// See <https://developer.tomba.io/#create-a-lead>
    pub fn create_lead(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "leads", body)
    }

    /// Update an existing lead.
    ///
    /// See <https://developer.tomba.io/#update-a-lead>
    pub fn update_lead(
        &self,
        id: &str,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        let path = format!("leads/{}", id);
        self.call_json("PUT", &path, body)
    }

    /// Delete a lead by ID.
    ///
    /// See <https://developer.tomba.io/#delete-a-lead>
    pub fn delete_lead(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("leads/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }
}
