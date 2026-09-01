use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// List all saved leads.
    ///
    /// See <https://docs.tomba.io/api/leads>
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
    /// See <https://docs.tomba.io/api/leads#get-lead>
    pub fn get_lead(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("leads/{}", id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new lead.
    ///
    /// See <https://docs.tomba.io/api/leads#create-lead>
    pub fn create_lead(
        &self,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        self.call_json("POST", "leads", body)
    }

    /// Update an existing lead.
    ///
    /// See <https://docs.tomba.io/api/leads#update-lead>
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
    /// See <https://docs.tomba.io/api/leads#delete-lead>
    pub fn delete_lead(&self, id: &str) -> Result<TombaResponse, TombaError> {
        let path = format!("leads/{}", id);
        self.call("DELETE", &path, &HashMap::new())
    }
}
