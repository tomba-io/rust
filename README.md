# [<img src="https://tomba.io/logo.svg" alt="Tomba" width="25"/>](https://tomba.io/) Tomba Email Finder Rust Client Library

This is the official Rust client library for the [Tomba.io](https://tomba.io) Email Finder API,
allowing you to:

- [Domain Search](https://tomba.io/domain-search) (Search emails are based on the website You give one domain name and it returns all the email addresses found on the internet.)
- [Email Finder](https://tomba.io/email-finder) (This API endpoint generates or retrieves the most likely email address from a domain name, a first name and a last name..)
- [Email Verifier](https://tomba.io/email-verifier) (checks the deliverability of a given email address, verifies if it has been found in our database, and returns their sources.)
- [Email Enrichment.](https://tomba.io/enrichment) (Locate and include data in your emails.)
- [Author Finder.](https://tomba.io/author-finder) (Instantly discover the email addresses of article authors.)
- [LinkedIn Finder.](https://tomba.io/linkedin-finder) (Instantly discover the email addresses of Linkedin URLs.)

## Features

- Collect publicly available emails online (Html, execute JavaScript,files,).
- No duplicate email No duplicate domain .
- No webmail like Gmail,Outlook and the others.
- We detect 15 type of hashes and remove them.
- No disposable and temporary email address.

## Getting Started

You'll need an Tomba API access token, which you can get by signing up for a free account at [https://app.tomba.io/auth/register](https://app.tomba.io/auth/register)

The free plan is limited to 25 search request and 50 verification a month, To enable all the data fields and additional request volumes see [https://tomba.io/pricing](https://tomba.io/pricing).

## Installation

Add this to your package's `Cargo.toml` file:

```toml
[dependencies]
tomba = "0.1"
```

## Usage

### Domain Search

get email addresses found on the internet.

```rust
use tomba::{Tomba, TombaConfig};

let config = TombaConfig {
   key: "ta_xxxx".to_string(),
   secret: "ts_xxxx".to_string(),
};

let mut tomba = Tomba::init(config).expect("should construct");

let res = tomba.domain_search("tomba.io".to_string()).expect("should do domain_search ");

println!("website country {:?}", res.data.organization.location.country);
```

### Email Finder

Find the verified email address of any professional.

```rust
use tomba::{Tomba, TombaConfig};

let config = TombaConfig {
   key: "ta_xxxx".to_string(),
   secret: "ts_xxxx".to_string(),
};

let mut tomba = Tomba::init(config).expect("should construct");

let res = tomba.email_finder("zapier.com".to_string(), "F_NAME".to_string(), "L_NAME".to_string()).expect("should do email finder");

println!("Email Finder email {}", res.data.email)
```

### Email Verifier

Verify the validity of any professional email address with the most complete email checker.

```rust
use tomba::{Tomba, TombaConfig};

let config = TombaConfig {
   key: "ta_xxxx".to_string(),
   secret: "ts_xxxx".to_string(),
};

let mut tomba = Tomba::init(config).expect("should construct");

let res = tomba.email_verifier("b.mohamed@tomba.io".to_string()).expect("should do verify");

println!("Email status {}", res.data.email.status)
```

## Documentation

See the [official documentation](https://docs.tomba.io/introduction).

### Other Libraries

There are official Tomba Email Finder client libraries available for many languages including PHP, Python, Go, Java, Ruby, and many popular frameworks such as Django, Rails and Laravel. There are also many third party libraries and integrations available for our API.

[https://docs.tomba.io/libraries](https://docs.tomba.io/libraries)

### About Tomba

Founded in 2020, Tomba prides itself on being the most reliable, accurate, and in-depth source of Email address data available anywhere. We process terabytes of data to produce our Email finder API, company.

[![image](https://avatars.githubusercontent.com/u/67979591?s=200&v=4)](https://tomba.io/)

## Contribution

1. Fork it (<https://github.com/tomba-io/rust/fork>)
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

## License

Please see the [Apache 2.0 license](http://www.apache.org/licenses/LICENSE-2.0.html) file for more information.
