//   Copyright 2021 Tomba technology web service LLC
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use crate::{
    Account, Autocomplete, Count, Finder, Logs, Search, Sources, Status,
    Usage, Verifier,DEFAULT_BASE_URL, ACCOUNT_PATH, AUTOCOMPLETE_PATH, COUNT_PATH, FINDER_PATH,
    LOGS_PATH, SEARCH_PATH, SOURCES_PATH, STATUS_PATH, USAGE_PATH,
    VERIFIER_PATH,
};
use reqwest;
use serde_json::{json, Value};

/// TombaConfig structure configuration.
pub struct TombaConfig {
    /// Tomba api key.
    pub key: String,

    /// Tomba secret key.
    pub secret: String,
}

/// Tomba requests context structure.
pub struct Tomba {
    url: String,
    key: String,
    secret: String,
}

/// Tomba Email finder
impl Tomba {
    /// Tomba Construct.
    ///
    /// # Examples
    ///
    /// ```
    /// use tomba::TombaConfig;
    ///
    /// let config = TombaConfig { key: "my key".to_string(), secret: "my secret".to_string(),  };
    /// ```
    pub fn init(
        _config: TombaConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            url: DEFAULT_BASE_URL.to_owned(),
            key: _config.key,
            secret: _config.secret,
        })
    }

    /// Tomba http Client
    ///
    /// # Arguments
    ///
    /// * `_path` - A string specific path.
    pub fn call(
        &mut self,
        _path: String,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let client = reqwest::Client::builder().build()?;
        // Perform the actual execution of the network request
        let misses: Vec<&str> = vec![];
        let url = format!("{base}{_path}", _path = _path, base = self.url);

        let res = client
            .get(&url)
            .header("X-Tomba-Key", &self.key)
            .header("X-Tomba-Secret", &self.secret)
            .header("x-Sdk-Version", "tomba:rust:v1.0.0")
            .json(&json!(misses))
            .send()?;

        // Check request status
        if res.status() != 200 {
            // Throw error
        };

        // Acquire response
        let raw_resp = res.error_for_status()?.text()?;

        // Parse the response
        let resp: Value = serde_json::from_str(&raw_resp)?;

        Ok(resp)
    }

    /// Returns information about the current account.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.account().expect("should do account");
    ///
    /// // println!("account email {}", res.data.email)
    /// 
    /// // account email b.mohamed@tomba.io
    /// ```
    pub fn account(&mut self) -> Result<Account, Box<dyn std::error::Error>> {
        let call =
            self.call(ACCOUNT_PATH.to_string()).expect("should do call");
        let details: Account = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Search emails are based on the website You give one domain name and it returns all the email addresses found on the internet.
    ///
    /// # Arguments
    ///
    /// * `_domain` - A string domain Domain name from which you want to find the email addresses. For example, "stripe.com".
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.domain_search("tomba.io".to_string()).expect("should do domain_search ");
    /// 
    /// // println!("website country {:?}", res.data.organization.location.country);
    /// 
    /// // website country Some("US")
    /// ```
    pub fn domain_search(
        &mut self,
        _domain: String,
    ) -> Result<Search, Box<dyn std::error::Error>> {
        let url = format!("{p}{d}", p = SEARCH_PATH, d = _domain);
        let call = self.call(url).expect("should do call");
        let details: Search = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Returns total email addresses we have for one domain.
    ///
    /// # Arguments
    ///
    /// * `_domain` - A string domain name from which you want to find the email addresses. For example, "stripe.com".
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.count("tomba.io".to_string()).expect("should count");
    /// 
    /// // println!("total email on website {}", res.data.total)
    /// 
    /// // total email on website 14
    /// ```
    pub fn count(
        &mut self,
        _domain: String,
    ) -> Result<Count, Box<dyn std::error::Error>> {
        let url = format!("{p}?domain={d}", p = COUNT_PATH, d = _domain);
        let call = self.call(url).expect("should do call");
        let details: Count = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Returns domain status if is webmail or disposable.
    ///
    /// # Arguments
    ///
    /// * `_domain` - A string domain name from which you want to check. For example, "gmail.com".
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.status("gmail.com".to_string()).expect("should do status");
    /// 
    /// // println!("the website gmail.com is webmail {}", res.webmail)
    /// 
    /// // the website gmail.com is webmail true
    /// ```
    pub fn status(
        &mut self,
        _domain: String,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let url = format!("{p}?domain={d}", p = STATUS_PATH, d = _domain);
        let call = self.call(url).expect("should do call");
        let details: Status = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Company Autocomplete is an API that lets you auto-complete company names and retrieve logo and domain information.
    ///
    /// # Arguments
    ///
    /// * `_search` - A string name company or website.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.autocomplete("tomba.io".to_string()).expect("should do autocomplete");
    /// 
    /// // println!("website name {:?} and {:?} emails", res.data[0].name, res.data[0].email_count)
    /// 
    /// // website name Some("Tomba") and Some(14) emails
    /// ```
    pub fn autocomplete(
        &mut self,
        _search: String,
    ) -> Result<Autocomplete, Box<dyn std::error::Error>> {
        let url = format!("{p}?query={s}", p = AUTOCOMPLETE_PATH, s = _search);
        let call = self.call(url).expect("should do call");
        let details: Autocomplete = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Generates or retrieves the most likely email address from a domain name, a first name and a last name.
    ///
    /// # Arguments
    ///
    /// * `_domain` - A string domain name of the company, used for emails. For example, "tomba.com".
    /// * `_fname` - A string The person's first name. It doesn't need to be in lowercase.
    /// * `_lname` - A string The person's last name. It doesn't need to be in lowercase.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.email_finder("zapier.com".to_string(), "simon".to_string(), "charette".to_string()).expect("should do email finder");
    /// 
    /// // println!("Email Finder email {}", res.data.email)
    /// 
    /// // Email Finder email simon.charette@zapier.co
    /// ```
    pub fn email_finder(
        &mut self,
        _domain: String,
        _fname: String,
        _lname: String,
    ) -> Result<Finder, Box<dyn std::error::Error>> {
        let url = format!(
            "{p}{s}?first_name={f}&last_name={l}",
            p = FINDER_PATH,
            s = _domain,
            f = _fname,
            l = _lname
        );
        let call = self.call(url).expect("should do call");
        let details: Finder = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Verify the deliverability of an email address.
    ///
    /// # Arguments
    ///
    /// * `_email` - A string email address you want to verify.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.email_verifier("b.mohamed@tomba.io".to_string()).expect("should do verify");
    /// 
    /// // println!("Email status {}", res.data.email.status)
    /// 
    /// // Email status valid
    /// ```
    pub fn email_verifier(
        &mut self,
        _email: String,
    ) -> Result<Verifier, Box<dyn std::error::Error>> {
        let url = format!("{p}{d}", p = VERIFIER_PATH, d = _email);
        let call = self.call(url).expect("should do call");
        let details: Verifier = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Find email address source somewhere on the web.
    ///
    /// # Arguments
    ///
    /// * `_email` - A string email address you want to find sources.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.email_sources("b.mohamed@tomba.io".to_string()).expect("should find sources");
    /// 
    /// // println!("first source url {} extracted on {}", res.data[0].uri, res.data[0].extracted_on)
    /// 
    /// // first source url https://github.com/tomba-io/generic-emails/blob/084fc1a63d3cdaf9a34f255bedc2baea49a8e8b9/src/lib/validation/hash.ts extracted on 2021-02-08T20:09:54+01:0
    /// ```
    pub fn email_sources(
        &mut self,
        _email: String,
    ) -> Result<Sources, Box<dyn std::error::Error>> {
        let url = format!("{p}{d}", p = SOURCES_PATH, d = _email);
        let call = self.call(url).expect("should do call");
        let details: Sources = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Check your monthly requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.usage().expect("should do usage");
    /// 
    /// // println!("total usage on Domain search {}", res.total.domain)
    /// 
    /// // total usage on Domain search 2615
    /// ```
    pub fn usage(&mut self) -> Result<Usage, Box<dyn std::error::Error>> {
        let call = self.call(USAGE_PATH.to_string()).expect("should do call");
        let details: Usage = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }

    /// Returns a your last 1,000 requests you made during the last 3 months.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tomba::{Tomba, TombaConfig};
    /// 
    /// let config = TombaConfig {
    ///    key: "ta_xxxx".to_string(),
    ///    secret: "ts_xxxx".to_string(),
    /// };
    /// 
    /// let mut tomba = Tomba::init(config).expect("should construct");
    /// 
    /// let res = tomba.logs().expect("should do logs");
    /// 
    /// // println!("Requests to {} from country {}", res.data[0].uri, res.data[0].country)
    /// // Requests to https://api.tomba/v1/domain-search/tomba.io from country DZ
    /// ```
    pub fn logs(&mut self) -> Result<Logs, Box<dyn std::error::Error>> {
        let call = self.call(LOGS_PATH.to_string()).expect("should do call");
        let details: Logs = serde_json::from_str(&call.to_string())?;
        Ok(details)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use TombaConfig;

    #[test]
    fn tomba_config() {
        let config = TombaConfig {
            key: "key".to_string(),
            secret: "secret".to_string(),
        };

        assert_eq!(config.key, "key");
        assert_eq!(config.secret, "secret");
    }
}
