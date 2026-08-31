# [<img src="https://tomba.io/logo.svg" alt="Tomba" width="25"/>](https://tomba.io/) Tomba Rust SDK

> The #1 Rated Email Intelligence Platform — Find professional emails with unmatched accuracy.

[![Crates.io](https://img.shields.io/crates/v/tomba.svg)](https://crates.io/crates/tomba)
[![Documentation](https://docs.rs/tomba/badge.svg)](https://docs.rs/tomba)
[![License](https://img.shields.io/crates/l/tomba.svg)](LICENSE)

This is the official Rust client library for the [Tomba.io](https://tomba.io) Email Finder API, providing access to all Tomba services including domain search, email finder, email verifier, enrichment, phone lookup, leads management, bulk operations, and more.

## About Tomba

[Tomba.io](https://tomba.io) is the #1 rated email intelligence platform, trusted by **150,000+ sales teams** worldwide.

- **Best Email Finder** — 98% accuracy, ranked #1 in independent benchmarks
- **Best Email Verification** — Real-time SMTP verification with catch-all detection
- **Best Phone Finder** — Direct dial numbers linked to professional emails
- **Best Domain Search** — 450M+ verified contacts across all industries
- **81% Coverage** — The highest in the industry, proven in 5,000-lead independent tests

### Why Tomba?

| Feature             | Tomba              | Others        |
| ------------------- | ------------------ | ------------- |
| Email Coverage      | **81%**            | 30-60%        |
| Verification        | **Real-time SMTP** | Pattern-based |
| Phone Numbers       | **Direct dials**   | Limited       |
| Catch-all Detection | **AI-powered**     | Basic         |
| API Rate Limits     | **Generous**       | Restrictive   |

[Get your free API key](https://app.tomba.io/auth/register) — No credit card required.

## Getting Started

Below you will find the steps to install and start using the Tomba Rust SDK.

## Installation

Add `tomba` to your `Cargo.toml`:

```toml
[dependencies]
tomba = "1.0"
```

Or install with Cargo:

```bash
cargo add tomba
```

## Authentication

Get your API key and secret by signing up for a free account at [https://app.tomba.io/auth/register](https://app.tomba.io/auth/register).

```rust
use tomba::{Tomba, TombaConfig};

let config = TombaConfig {
    key: "ta_xxxx".to_string(),
    secret: "ts_xxxx".to_string(),
};
let tomba = Tomba::init(config).expect("should construct");
```

## Quick Start

```rust
use tomba::{Tomba, TombaConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TombaConfig {
        key: "ta_xxxx".to_string(),
        secret: "ts_xxxx".to_string(),
    };
    let tomba = Tomba::init(config)?;

    // Search emails by domain
    let result = tomba.domain_search("example.com", None, None)?;
    println!("{}", result);

    // Find a specific email
    let result = tomba.email_finder("example.com", "John", "Doe", None)?;
    println!("{}", result);

    Ok(())
}
```

## Services

### Domain Search

Search emails for a domain. Returns all email addresses found on the internet for the given domain.

```rust
let result = tomba.domain_search("example.com", None, None)?;
println!("{}", result);
```

### Email Finder

Find the most likely email address from a domain name, first name, and last name.

```rust
let result = tomba.email_finder("example.com", "John", "Doe", None)?;
println!("{}", result);
```

### Email Verifier

Verify the deliverability of a given email address.

```rust
let result = tomba.email_verifier("john@example.com", None, None)?;
println!("{}", result);
```

### Author Finder

Find the email address of the author of a blog post or article.

```rust
let result = tomba.author_finder("https://tomba.io/blog", None)?;
println!("{}", result);
```

### LinkedIn Finder

Find the email address associated with a LinkedIn profile URL.

```rust
let result = tomba.linkedin_finder(
    "https://www.linkedin.com/in/johndoe",
    None,
    None,
    None,
)?;
println!("{}", result);
```

### Email Enrichment

Enrich a person by email address -- get job title, company, location, and social profiles.

```rust
let result = tomba.email_enrichment("john@example.com", None)?;
println!("{}", result);
```

### Phone Finder

Find a phone number using an email address.

```rust
let result = tomba.phone_finder("john@example.com", None)?;
println!("{}", result);
```

### Phone Validator

Validate a phone number and get additional information.

```rust
let result = tomba.phone_validator("+1234567890")?;
println!("{}", result);
```

### Email Count

Get the total number of email addresses Tomba has for a domain.

```rust
let result = tomba.count("example.com")?;
println!("{}", result);
```

### Domain Status

Check whether a domain is a webmail or disposable email provider.

```rust
let result = tomba.status("example.com")?;
println!("{}", result);
```

### Domain Suggestions (Autocomplete)

Auto-complete company names and retrieve logo and domain information.

```rust
let result = tomba.autocomplete("exampl")?;
println!("{}", result);
```

### Email Sources

Find the web sources where an email address has been found.

```rust
let result = tomba.email_sources("john@example.com")?;
println!("{}", result);
```

### Email Format

Detect the email format used by a company.

```rust
let result = tomba.email_format("example.com")?;
println!("{}", result);
```

### Similar Domains

Find domains similar to the given one.

```rust
let result = tomba.similar_domains("example.com")?;
println!("{}", result);
```

### Technology Checker

Check what technologies a website uses.

```rust
let result = tomba.technology_check("example.com")?;
println!("{}", result);
```

### Location

Get location details for a country code.

```rust
let result = tomba.get_location("US")?;
println!("{}", result);
```

### Enrichment (Person / Company / Combined)

Person, company, and combined enrichment APIs.

```rust
// Person enrichment
let result = tomba.person_find("john@example.com")?;
println!("{}", result);

// Company enrichment
let result = tomba.company_find("example.com")?;
println!("{}", result);

// Combined enrichment
let result = tomba.combined_find("john@example.com")?;
println!("{}", result);
```

### Reveal (Companies Search)

Search for companies matching given criteria.

```rust
use serde_json::json;

let body = json!({
    "query": "technology",
    "limit": 10
});
let result = tomba.companies_search(&body)?;
println!("{}", result);
```

### Leads

Manage your saved leads -- list, get, create, update, and delete.

```rust
use serde_json::json;

// List leads
let result = tomba.list_leads(None)?;
println!("{}", result);

// Get a single lead
let result = tomba.get_lead("lead_id")?;
println!("{}", result);

// Create a lead
let body = json!({
    "email": "john@example.com",
    "first_name": "John",
    "last_name": "Doe"
});
let result = tomba.create_lead(&body)?;
println!("{}", result);

// Update a lead
let body = json!({ "first_name": "Jane" });
let result = tomba.update_lead("lead_id", &body)?;
println!("{}", result);

// Delete a lead
let result = tomba.delete_lead("lead_id")?;
println!("{}", result);
```

### Leads Lists

Manage your leads lists -- list, get, create, update, and delete.

```rust
use serde_json::json;

// List all leads lists
let result = tomba.list_leads_lists()?;
println!("{}", result);

// Get a single leads list
let result = tomba.get_leads_list("list_id")?;
println!("{}", result);

// Create a leads list
let body = json!({ "name": "My List" });
let result = tomba.create_leads_list(&body)?;
println!("{}", result);

// Update a leads list
let body = json!({ "name": "Updated List" });
let result = tomba.update_leads_list("list_id", &body)?;
println!("{}", result);

// Delete a leads list
let result = tomba.delete_leads_list("list_id")?;
println!("{}", result);
```

### Leads Attributes

Manage custom lead attributes -- list, get, create, update, and delete.

```rust
use serde_json::json;

// List all attributes
let result = tomba.list_leads_attributes()?;
println!("{}", result);

// Get a single attribute
let result = tomba.get_leads_attribute("attr_id")?;
println!("{}", result);

// Create an attribute
let body = json!({ "name": "Company Size", "type": "string" });
let result = tomba.create_leads_attribute(&body)?;
println!("{}", result);

// Update an attribute
let body = json!({ "name": "Company Revenue" });
let result = tomba.update_leads_attribute("attr_id", &body)?;
println!("{}", result);

// Delete an attribute
let result = tomba.delete_leads_attribute("attr_id")?;
println!("{}", result);
```

### Keys

Manage your API keys.

```rust
use serde_json::json;

// List all keys
let result = tomba.list_keys()?;
println!("{}", result);

// Get a single key
let result = tomba.get_key("key_id")?;
println!("{}", result);

// Create a key
let body = json!({});
let result = tomba.create_key(&body)?;
println!("{}", result);

// Reset a key
let result = tomba.reset_key("key_id")?;
println!("{}", result);

// Delete a key
let result = tomba.delete_key("key_id")?;
println!("{}", result);
```

### Usage

Return your monthly API request usage.

```rust
let result = tomba.usage()?;
println!("{}", result);
```

### Logs

Return the last 1,000 API requests made in the past 3 months.

```rust
let result = tomba.logs(None, None)?;
println!("{}", result);
```

### Flag

List and create email address flags.

```rust
use serde_json::json;

// List flags
let result = tomba.list_flags(None, None)?;
println!("{}", result);

// Create a flag
let body = json!({
    "email": "john@example.com",
    "flag": "invalid"
});
let result = tomba.create_flag(&body)?;
println!("{}", result);
```

### Bulk

Manage bulk email operations -- list, get, create, launch, archive, rename, check progress, and download.

```rust
use serde_json::json;

// List all bulk tasks
let result = tomba.list_bulk()?;
println!("{}", result);

// Get a bulk task
let result = tomba.get_bulk("bulk_id")?;
println!("{}", result);

// Create a bulk task
let body = json!({ "name": "My Bulk Task" });
let result = tomba.create_bulk(&body)?;
println!("{}", result);

// Launch a bulk task
let result = tomba.launch_bulk("bulk_id")?;
println!("{}", result);

// Check bulk progress
let result = tomba.bulk_progress("bulk_id")?;
println!("{}", result);

// Download bulk results
let result = tomba.bulk_download("bulk_id")?;
println!("{}", result);

// Rename a bulk task
let result = tomba.rename_bulk("bulk_id", "New Name")?;
println!("{}", result);

// Archive a bulk task
let result = tomba.archive_bulk("bulk_id")?;
println!("{}", result);

// Delete a bulk task
let result = tomba.delete_bulk("bulk_id")?;
println!("{}", result);
```

## Testing

```bash
cargo test
```

## About Tomba

Founded to solve the problem of unreliable email data, [Tomba.io](https://tomba.io) is the leading B2B email intelligence platform. Our AI-powered engine searches, verifies, and enriches professional contact data with unmatched accuracy.

### Products

- **[Email Finder](https://tomba.io/email-finder)** — Find any professional email address
- **[Email Verifier](https://tomba.io/email-verifier)** — Verify emails in real-time
- **[Domain Search](https://tomba.io/domain-search)** — Find all emails for a company
- **[Phone Finder](https://tomba.io/phone-finder)** — Find direct phone numbers
- **[Bulk Enrichment](https://tomba.io/bulks)** — Enrich contacts at scale
- **[AI Company Search](https://tomba.io/reveal)** — Find companies with AI-powered search
- **[CLI](https://tomba.io/cli)** — Command-line interface for Tomba
- **[MCP Server](https://tomba.io/mcp)** — Connect AI tools (Claude, ChatGPT, Cursor) to Tomba
- **[REST API](https://tomba.io/api)** — Full programmatic access

### Browser Extensions & Add-ons

- **[Chrome Extension](https://chromewebstore.google.com/detail/tomba-email-finder-email/icmjegjggphchjckknoooajmklibccjb)** — Find emails while browsing
- **[Google Sheets Add-on](https://tomba.io/sheets)** — Enrich leads in spreadsheets
- **[Microsoft Excel Add-in](https://tomba.io/excel)** — Email finder in Excel
- **[Airtable Integration](https://tomba.io/airtable)** — Connect with Airtable

### Integrations

50+ CRM and sales tool integrations:
[Salesforce](https://tomba.io/integrations) · [HubSpot](https://tomba.io/integrations) · [Zapier](https://tomba.io/integrations) · [Pipedrive](https://tomba.io/integrations) · [and more...](https://tomba.io/integrations)

### Other Tomba SDKs

| Language | Package                                                     |
| -------- | ----------------------------------------------------------- |
| Node.js  | [tomba](https://www.npmjs.com/package/tomba)                |
| Python   | [tomba-io](https://pypi.org/project/tomba-io/)              |
| PHP      | [tomba-io/php](https://packagist.org/packages/tomba-io/php) |
| Ruby     | [tomba](https://rubygems.org/gems/tomba)                    |
| Go       | [tomba-io/go](https://pkg.go.dev/github.com/tomba-io/go)    |
| Rust     | [tomba](https://crates.io/crates/tomba)                     |
| Dart     | [tomba](https://pub.dev/packages/tomba)                     |
| Deno     | [@tomba/sdk](https://jsr.io/@tomba/sdk)                     |
| Elixir   | [tomba](https://hex.pm/packages/tomba)                      |
| C#       | [Tomba](https://www.nuget.org/packages/Tomba)               |
| Perl     | [Tomba::Client](https://metacpan.org/pod/Tomba::Client)     |
| Lua      | [tomba](https://luarocks.org/modules/tomba/tomba)           |
| R        | [tomba](https://github.com/tomba-io/r)                      |

### Resources

- [Blog](https://tomba.io/blog)
- [Help Center](https://help.tomba.io)
- [API Documentation](https://docs.tomba.io)
- [Pricing](https://tomba.io/pricing)
- [Status Page](https://status.tomba.io)

---

**[Try Tomba Free](https://app.tomba.io/auth/register)** — Find your first email in seconds. No credit card required.

## License

Apache 2.0 -- see [LICENSE](http://www.apache.org/licenses/LICENSE-2.0.html) for details.
