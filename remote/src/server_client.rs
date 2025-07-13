use crate::error::{Result, WavewatchError};
use crate::rf_explorer::TraceData;
use log::{debug, error, info, warn};
use reqwest::Client;
use std::time::Duration;

pub struct ServerClient {
    client: Client,
    base_url: String,
}

impl ServerClient {
    /// Create a new server client
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        info!("Created server client for: {}", base_url);

        ServerClient { client, base_url }
    }

    /// Send trace data to the server
    pub async fn send_trace(&self, analyzer_id: &str, trace_data: &TraceData) -> Result<()> {
        let url = format!("{}/analyzers/{}/traces", self.base_url, analyzer_id);

        debug!(
            "Sending trace data to {} (frequencies: {}, amplitudes: {})",
            url,
            trace_data.frequencies_hz.len(),
            trace_data.amplitudes_dbm.len()
        );

        let response = self.client.post(&url).json(trace_data).send().await?;

        let status = response.status();
        if status.is_success() {
            debug!("Successfully sent trace data to server");
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            error!("Server responded with status {}: {}", status, error_text);
            Err(WavewatchError::Config(format!(
                "HTTP {}: {}",
                status, error_text
            )))
        }
    }

    /// Send trace data with retry logic
    pub async fn send_trace_with_retry(
        &self,
        analyzer_id: &str,
        trace_data: &TraceData,
        max_retries: u32,
    ) -> Result<()> {
        let mut attempts = 0;

        loop {
            match self.send_trace(analyzer_id, trace_data).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    attempts += 1;

                    if attempts >= max_retries {
                        error!(
                            "Failed to send trace data after {} attempts: {}",
                            max_retries, e
                        );
                        return Err(e);
                    }

                    let delay = Duration::from_millis(1000 * attempts as u64);
                    warn!(
                        "Failed to send trace data (attempt {}/{}): {}. Retrying in {:?}",
                        attempts, max_retries, e, delay
                    );

                    tokio::time::sleep(delay).await;
                }
            }
        }
    }
}
