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

    pub fn log(&self, logs: &[Log]) -> bool {
        match self.send(LogRequest::new(logs)) {
            Err(err) => {
                println!("[dlog::internal] Failed to send logs: {}", err);
                false
            }
            Ok(val) if !val.status().is_success() => {
                println!("[dlog::internal] An error occurred: {}", val.text().unwrap());
                false
            }
            _ => true,
        }
    }

    fn send<T: serde::Serialize + Sized>(&self, request: T) -> Result<reqwest::blocking::Response, reqwest::Error> {
        self.client
            .post("https://log.dlog.cloud")
            .json(&request)
            .header("API_KEY", HeaderValue::from_str(&self.api_key).unwrap())
            .timeout(std::time::Duration::from_secs(10))
            .send()
    }
}
