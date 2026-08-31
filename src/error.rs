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

//! Error types for the Tomba SDK.

use std::fmt;

/// Represents all possible errors returned by the Tomba SDK.
#[derive(Debug)]
pub enum TombaError {
    /// An error returned by the Tomba API (e.g. 401, 429).
    Api {
        /// Human-readable error message from the API.
        message: String,
        /// HTTP status code.
        code: u16,
    },
    /// A transport-level HTTP error from reqwest.
    Http(reqwest::Error),
    /// A JSON serialization / deserialization error.
    Parse(serde_json::Error),
    /// An invalid parameter was provided.
    InvalidParam(String),
}

impl std::error::Error for TombaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TombaError::Http(e) => Some(e),
            TombaError::Parse(e) => Some(e),
            TombaError::Api { .. } => None,
            TombaError::InvalidParam(_) => None,
        }
    }
}

impl fmt::Display for TombaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TombaError::Api { message, code } => {
                write!(f, "Tomba API error ({}): {}", code, message)
            }
            TombaError::Http(e) => {
                write!(f, "HTTP request error: {}", e)
            }
            TombaError::Parse(e) => {
                write!(f, "JSON parse error: {}", e)
            }
            TombaError::InvalidParam(msg) => {
                write!(f, "Invalid parameter: {}", msg)
            }
        }
    }
}

impl From<reqwest::Error> for TombaError {
    fn from(err: reqwest::Error) -> Self {
        TombaError::Http(err)
    }
}

impl From<serde_json::Error> for TombaError {
    fn from(err: serde_json::Error) -> Self {
        TombaError::Parse(err)
    }
}
