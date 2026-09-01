use std::collections::HashMap;

use serde_json::Value;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

const VALID_BULK_TYPES: &[&str] = &[
    "search",
    "similar",
    "company",
    "finder",
    "enrich",
    "linkedin",
    "author",
    "verifier",
    "phone-finder",
    "phone-validator",
];

fn validate_bulk_type(bulk_type: &str) -> Result<(), TombaError> {
    if !VALID_BULK_TYPES.contains(&bulk_type) {
        return Err(TombaError::InvalidParam(format!(
            "Invalid bulk type: \"{}\". Must be one of: {}",
            bulk_type,
            VALID_BULK_TYPES.join(", ")
        )));
    }
    Ok(())
}

impl Tomba {
    /// List all bulk tasks of a given type.
    ///
    /// See <https://docs.tomba.io/api/bulks>
    pub fn list_bulk(
        &self,
        bulk_type: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}", bulk_type);
        self.call("GET", &path, &HashMap::new())
    }

    /// Get a single bulk task by type and ID.
    ///
    /// See <https://docs.tomba.io/api/bulks#get-bulk>
    pub fn get_bulk(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}", bulk_type, id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Create a new bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#create-bulk>
    pub fn create_bulk(
        &self,
        bulk_type: &str,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}", bulk_type);
        self.call_json("POST", &path, body)
    }

    /// Launch a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#launch-bulk>
    pub fn launch_bulk(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}", bulk_type, id);
        self.call_json("PUT", &path, &serde_json::json!({}))
    }

    /// Delete a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#delete-bulk>
    pub fn delete_bulk(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}/delete", bulk_type, id);
        self.call("DELETE", &path, &HashMap::new())
    }

    /// Archive a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#archive-bulk>
    pub fn archive_bulk(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}/archive", bulk_type, id);
        self.call("DELETE", &path, &HashMap::new())
    }

    /// Rename a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#rename-bulk>
    pub fn rename_bulk(
        &self,
        bulk_type: &str,
        id: &str,
        name: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}/rename", bulk_type, id);
        self.call_json("PUT", &path, &serde_json::json!({ "name": name }))
    }

    /// Get progress of a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#bulk-progress>
    pub fn bulk_progress(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}/progress", bulk_type, id);
        self.call("GET", &path, &HashMap::new())
    }

    /// Download results of a bulk task.
    ///
    /// See <https://docs.tomba.io/api/bulks#download-bulk>
    pub fn bulk_download(
        &self,
        bulk_type: &str,
        id: &str,
    ) -> Result<TombaResponse, TombaError> {
        validate_bulk_type(bulk_type)?;
        let path = format!("bulk/{}/{}/download", bulk_type, id);
        self.call("GET", &path, &HashMap::new())
    }
}
