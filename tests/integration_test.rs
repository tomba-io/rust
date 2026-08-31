// Integration tests for the Tomba Rust SDK.
//
// Unit tests (no network) run by default.
// Live integration tests (marked #[ignore]) require TOMBA_API_KEY and
// TOMBA_SECRET_KEY environment variables and hit the real API.
//
// Run all tests:       cargo test
// Run live tests only: cargo test -- --ignored

use tomba::{Tomba, TombaConfig, TombaError};

// ------------------------------------------------------------------
// Unit tests (no network required)
// ------------------------------------------------------------------

#[test]
fn test_init_success() {
    let config = TombaConfig {
        key: "ta_abc".to_string(),
        secret: "ts_xyz".to_string(),
    };
    let tomba = Tomba::init(config);
    assert!(tomba.is_ok());
}

#[test]
fn test_error_display_api() {
    let err = TombaError::Api {
        message: "Unauthorized".to_string(),
        code: 401,
    };
    let display = format!("{}", err);
    assert!(display.contains("401"));
    assert!(display.contains("Unauthorized"));
}

#[test]
fn test_error_display_parse() {
    let bad_json = serde_json::from_str::<serde_json::Value>("not json");
    let err: TombaError = bad_json.unwrap_err().into();
    let display = format!("{}", err);
    assert!(display.contains("JSON parse error"));
}

#[test]
fn test_error_debug() {
    let err = TombaError::Api {
        message: "test".into(),
        code: 500,
    };
    let debug = format!("{:?}", err);
    assert!(debug.contains("Api"));
}

#[test]
fn test_error_source_api() {
    use std::error::Error;
    let err = TombaError::Api {
        message: "msg".into(),
        code: 400,
    };
    assert!(err.source().is_none());
}

#[test]
fn test_error_from_serde() {
    let bad: Result<serde_json::Value, _> = serde_json::from_str("{invalid");
    let serde_err = bad.unwrap_err();
    let tomba_err: TombaError = serde_err.into();
    match tomba_err {
        TombaError::Parse(_) => {} // expected
        other => panic!("expected Parse, got {:?}", other),
    }
}

// ------------------------------------------------------------------
// Live integration tests -- require TOMBA_API_KEY + TOMBA_SECRET_KEY env vars.
// Run with:  cargo test -- --ignored
// ------------------------------------------------------------------

fn live_client() -> Option<Tomba> {
    let key = std::env::var("TOMBA_API_KEY").ok()?;
    let secret = std::env::var("TOMBA_SECRET_KEY").ok()?;
    if key.is_empty() || secret.is_empty() {
        return None;
    }
    let config = TombaConfig { key, secret };
    Some(Tomba::init(config).expect("should construct"))
}

#[test]
#[ignore]
fn live_account() {
    if let Some(tomba) = live_client() {
        let res = tomba.account();
        assert!(res.is_ok(), "account failed: {:?}", res.err());
        let val = res.unwrap();
        assert!(val.data.get("data").is_some());
    }
}

#[test]
#[ignore]
fn live_domain_search() {
    if let Some(tomba) = live_client() {
        let res = tomba.domain_search("tomba.io", None, None);
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_count() {
    if let Some(tomba) = live_client() {
        let res = tomba.count("tomba.io");
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_status() {
    if let Some(tomba) = live_client() {
        let res = tomba.status("gmail.com");
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_autocomplete() {
    if let Some(tomba) = live_client() {
        let res = tomba.autocomplete("tomba");
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_email_finder() {
    if let Some(tomba) = live_client() {
        let res = tomba.email_finder("tomba.io", "Mohamed", "Ben", None);
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_email_verifier() {
    if let Some(tomba) = live_client() {
        let res = tomba.email_verifier("b.mohamed@tomba.io", None, None);
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_email_sources() {
    if let Some(tomba) = live_client() {
        let res = tomba.email_sources("b.mohamed@tomba.io");
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_usage() {
    if let Some(tomba) = live_client() {
        let res = tomba.usage();
        assert!(res.is_ok(), "{:?}", res.err());
    }
}

#[test]
#[ignore]
fn live_logs() {
    if let Some(tomba) = live_client() {
        let res = tomba.logs(None, None);
        assert!(res.is_ok(), "{:?}", res.err());
    }
}
