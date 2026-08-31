// Copyright 2021 Tomba technology web service LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::TombaError;
use crate::DEFAULT_BASE_URL;

const SDK_VERSION: &str = "tomba:rust:v1.0.0";

/// Rate-limit information extracted from response headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    /// Maximum requests allowed per second.
    pub x_second_rate_limit: Option<String>,
    /// Maximum requests allowed per minute.
    pub x_minute_rate_limit: Option<String>,
    /// Maximum requests allowed per day.
    pub x_daily_rate_limit: Option<String>,
    /// Remaining requests in the current minute window.
    pub x_minute_request_left: Option<String>,
    /// Remaining requests in the current daily window.
    pub x_daily_request_left: Option<String>,
    /// Seconds until the per-minute limit resets.
    pub x_minute_reset_seconds: Option<String>,
    /// Seconds until the daily limit resets.
    pub x_daily_reset_seconds: Option<String>,
    /// Standard `Retry-After` header value (seconds).
    pub retry_after: Option<String>,
    /// Standard `RateLimit-Policy` header value.
    pub rate_limit_policy: Option<String>,
    /// Standard `RateLimit` header value.
    pub rate_limit: Option<String>,
}

/// A response from the Tomba API containing the parsed JSON body
/// and rate-limit metadata from the response headers.
#[derive(Debug, Clone)]
pub struct TombaResponse {
    /// The parsed JSON response body.
    pub data: Value,
    /// Rate-limit information extracted from response headers.
    pub rate_limit: RateLimit,
}

/// Parse rate-limit headers from an HTTP response.
pub fn parse_rate_limit(headers: &reqwest::header::HeaderMap) -> RateLimit {
    let get = |name: &str| -> Option<String> {
        headers
            .get(name)
            .and_then(|v| v.to_str().ok())
            .map(String::from)
    };
    RateLimit {
        x_second_rate_limit: get("x-second-rate-limit"),
        x_minute_rate_limit: get("x-minute-rate-limit"),
        x_daily_rate_limit: get("x-daily-rate-limit"),
        x_minute_request_left: get("x-minute-request-left"),
        x_daily_request_left: get("x-daily-request-left"),
        x_minute_reset_seconds: get("x-minute-reset-seconds"),
        x_daily_reset_seconds: get("x-daily-reset-seconds"),
        retry_after: get("retry-after"),
        rate_limit_policy: get("ratelimit-policy"),
        rate_limit: get("ratelimit"),
    }
}

/// Configuration for the Tomba client.
pub struct TombaConfig {
    /// Tomba API key (starts with `ta_`).
    pub key: String,
    /// Tomba secret key (starts with `ts_`).
    pub secret: String,
}

/// The Tomba API client.
///
/// Create an instance with [`Tomba::init`], then call the endpoint
/// methods such as [`Tomba::account`], [`Tomba::domain_search`], etc.
pub struct Tomba {
    url: String,
    key: String,
    secret: String,
    client: Client,
}

impl Tomba {
    /// Create a new Tomba client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    ///
    /// let config = TombaConfig {
    ///     key: "ta_xxxx".to_string(),
    ///     secret: "ts_xxxx".to_string(),
    /// };
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// ```
    pub fn init(config: TombaConfig) -> Result<Self, TombaError> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()?;
        Ok(Self {
            url: DEFAULT_BASE_URL.to_owned(),
            key: config.key,
            secret: config.secret,
            client,
        })
    }

    // ------------------------------------------------------------------
    // HTTP helpers
    // ------------------------------------------------------------------

    /// Send a request with query parameters (GET / DELETE).
    ///
    /// * `method` -- `"GET"` or `"DELETE"`
    /// * `path`   -- path relative to the base URL, e.g. `"me"`
    /// * `params` -- query-string key/value pairs
    pub fn call(
        &self,
        method: &str,
        path: &str,
        params: &HashMap<String, String>,
    ) -> Result<TombaResponse, TombaError> {
        let url = format!("{}{}", self.url, path);

        let builder = match method {
            "DELETE" => self.client.delete(&url),
            _ => self.client.get(&url),
        };

        let resp = builder
            .header("X-Tomba-Key", &self.key)
            .header("X-Tomba-Secret", &self.secret)
            .header("Content-Type", "application/json")
            .header("x-Sdk-Version", SDK_VERSION)
            .query(params)
            .send()?;

        self.handle_response(resp)
    }

    /// Send a request with a JSON body (POST / PUT).
    ///
    /// * `method` -- `"POST"` or `"PUT"`
    /// * `path`   -- path relative to the base URL
    /// * `body`   -- JSON value to send as the request body
    pub fn call_json(
        &self,
        method: &str,
        path: &str,
        body: &Value,
    ) -> Result<TombaResponse, TombaError> {
        let url = format!("{}{}", self.url, path);

        let builder = match method {
            "PUT" => self.client.put(&url),
            _ => self.client.post(&url),
        };

        let resp = builder
            .header("X-Tomba-Key", &self.key)
            .header("X-Tomba-Secret", &self.secret)
            .header("Content-Type", "application/json")
            .header("x-Sdk-Version", SDK_VERSION)
            .json(body)
            .send()?;

        self.handle_response(resp)
    }

    /// Interpret the HTTP response, returning the parsed JSON body
    /// along with rate-limit headers, or a [`TombaError`].
    fn handle_response(
        &self,
        resp: reqwest::blocking::Response,
    ) -> Result<TombaResponse, TombaError> {
        let status = resp.status().as_u16();
        let rate_limit = parse_rate_limit(resp.headers());
        let body = resp.text()?;

        if status >= 400 {
            let message = serde_json::from_str::<Value>(&body)
                .ok()
                .and_then(|v| {
                    v.get("errors")
                        .and_then(|e| {
                            e.get(0)
                                .and_then(|e0| e0.get("message"))
                                .and_then(|m| m.as_str())
                                .map(String::from)
                        })
                        .or_else(|| {
                            v.get("message")
                                .and_then(|m| m.as_str())
                                .map(String::from)
                        })
                })
                .unwrap_or(body);

            return Err(TombaError::Api {
                message,
                code: status,
            });
        }

        let parsed: Value = serde_json::from_str(&body)?;
        Ok(TombaResponse {
            data: parsed,
            rate_limit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tomba_config() {
        let config = TombaConfig {
            key: "ta_key".to_string(),
            secret: "ts_secret".to_string(),
        };

        assert_eq!(config.key, "ta_key");
        assert_eq!(config.secret, "ts_secret");
    }

    #[test]
    fn test_tomba_init() {
        let config = TombaConfig {
            key: "ta_key".to_string(),
            secret: "ts_secret".to_string(),
        };
        let tomba = Tomba::init(config).expect("should construct");

        assert_eq!(tomba.key, "ta_key");
        assert_eq!(tomba.secret, "ts_secret");
        assert_eq!(tomba.url, DEFAULT_BASE_URL);
    }
}
