use reqwest::header::HeaderValue;

use crate::models::{Log, LogRequest, Priority};

#[derive(Debug)]
pub struct HttpIngestor {
    client: reqwest::Client,
    api_key: String,
}

impl HttpIngestor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }

    pub async fn has_valid_api_key(&self) -> bool {
        if let Ok(res) = self.send_async(LogRequest::new(&[])).await {
            if res.text().await.unwrap_or_default().contains("Invalid API_KEY") {
                return false
            }
        }
        return true
    }

    pub async fn check(&self) -> bool {
        match self.send_async(LogRequest::new(&[])).await {
            Ok(res) if res.status().is_success() => true,
            _ => false,
        }
    }

    pub async fn log_async(&self, logs: &[Log]) -> Result<(), Log> {
        match self.send_async(LogRequest::new(logs)).await {
            Err(err) => Err(Log::new(
                Priority::Critical,
                format!("[dlog] API connection error: {}", err),
            )),
            Ok(val) if !val.status().is_success() => Err(Log::new(
                Priority::Critical,
                format!("[dlog] Log ingestion failed: {}", val.text().await.unwrap_or_default()),
            )),
            _ => Ok(()),
        }
    }

    async fn send_async<T: serde::Serialize + Sized>(&self, request: T) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post("https://log.dlog.cloud")
            .json(&request)
            .header("API_KEY", HeaderValue::from_str(&self.api_key).unwrap())
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
    }
}
