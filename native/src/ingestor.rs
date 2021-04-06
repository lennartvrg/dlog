use reqwest::header::HeaderValue;

use crate::models::{Log, LogRequest};

#[derive(Debug)]
pub struct HttpIngestor {
    client: reqwest::blocking::Client,
    api_key: String,
}

impl HttpIngestor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
            api_key,
        }
    }

    pub fn log(&self, logs: Vec<Log>) {
        if let Err(err) = self
            .client
            .post("https://log.dlog.cloud")
            .json(&LogRequest { logs })
            .header("API_KEY", HeaderValue::from_str(&self.api_key).unwrap())
            .send()
        {
            println!("Failed to send logs: {}", err);
        }
    }
}
