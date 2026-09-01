use std::collections::HashMap;

use crate::error::TombaError;
use crate::tomba::{Tomba, TombaResponse};

impl Tomba {
    /// Find the most likely email address for a person at a company.
    ///
    /// See <https://docs.tomba.io/api/finder#email-finder>
    pub fn email_finder(
        &self,
        domain: &str,
        first_name: &str,
        last_name: &str,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("domain".into(), domain.into());
        params.insert("first_name".into(), first_name.into());
        params.insert("last_name".into(), last_name.into());
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "email-finder", &params)
    }

    /// Find the email address of the author of a blog post.
    ///
    /// See <https://docs.tomba.io/api/finder#author-finder>
    pub fn author_finder(
        &self,
        url: &str,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("url".into(), url.into());
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "author-finder", &params)
    }

    /// Find the email address from a LinkedIn URL.
    ///
    /// See <https://docs.tomba.io/api/finder#linkedin-finder>
    pub fn linkedin_finder(
        &self,
        url: &str,
        enrich_mobile: Option<bool>,
        full: Option<bool>,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("url".into(), url.into());
        if let Some(v) = enrich_mobile {
            params.insert("enrich_mobile".into(), v.to_string());
        }
        if let Some(v) = full {
            params.insert("full".into(), v.to_string());
        }
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "linkedin", &params)
    }

    /// Enrich a person by email address.
    ///
    /// See <https://docs.tomba.io/api/finder#email-enrichment>
    pub fn email_enrichment(
        &self,
        email: &str,
        webhook_url: Option<&str>,
    ) -> Result<TombaResponse, TombaError> {
        let mut params = HashMap::new();
        params.insert("email".into(), email.into());
        if let Some(v) = webhook_url {
            params.insert("webhook_url".into(), v.into());
        }
        self.call("GET", "enrichment", &params)
    }
}
