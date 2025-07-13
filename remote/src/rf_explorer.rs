use crate::error::{Result, WavewatchError};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceData {
    #[serde(rename = "FrequenciesHz")]
    pub frequencies_hz: Vec<u64>,
    #[serde(rename = "AmplitudesDbm")]
    pub amplitudes_dbm: Vec<f32>,
}

pub struct RfExplorerDevice {
    device: rfe::SpectrumAnalyzer,
    serial_number: String,
}

impl RfExplorerDevice {
    /// Connect to an RF Explorer device
    pub async fn connect(port: Option<String>) -> Result<Self> {
        let device = if let Some(port_path) = port {
            info!(
                "Attempting to connect to RF Explorer on port: {}",
                port_path
            );
            rfe::SpectrumAnalyzer::connect_with_name_and_baud_rate(&port_path, 500_000)
                .map_err(|e| WavewatchError::RfExplorerConnection(e))?
        } else {
            info!("Attempting to connect to RF Explorer with default settings...");
            rfe::SpectrumAnalyzer::connect().ok_or_else(|| {
                WavewatchError::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    "RF Explorer not found",
                ))
            })?
        };

        // Get the device's serial number to use as analyzer ID
        let serial_number = device.serial_number().ok_or_else(|| {
            WavewatchError::RfExplorerConnection(rfe::ConnectionError::DeviceInfoNotReceived)
        })?;

        info!(
            "Connected to RF Explorer with serial number: {}",
            serial_number
        );

        Ok(RfExplorerDevice {
            device,
            serial_number,
        })
    }

    /// Get the device's serial number (used as analyzer ID)
    pub fn get_analyzer_id(&self) -> &str {
        &self.serial_number
    }

    /// Configure the RF Explorer with frequency range
    pub fn configure(&mut self, start_freq: Option<u64>, stop_freq: Option<u64>) -> Result<()> {
        if let (Some(start), Some(stop)) = (start_freq, stop_freq) {
            info!("Configuring RF Explorer frequency range: {start} - {stop} Hz");
            self.device
                .set_start_stop(start, stop)
                .map_err(|e| WavewatchError::RfExplorer(e))?;
        } else {
            let start_freq_hz = self.device.start_freq().as_hz();
            let stop_freq_hz = self.device.stop_freq().as_hz();
            info!("Using frequency range: {start_freq_hz} - {stop_freq_hz} Hz");
        }

        Ok(())
    }

    /// Get the next sweep (trace) data from the RF Explorer
    pub fn get_trace(&mut self) -> Result<TraceData> {
        match self.device.wait_for_next_sweep() {
            Ok(amplitudes_dbm) => {
                let num_points = amplitudes_dbm.len();
                let start_freq = self.device.start_freq();
                let step = self.device.step_size();

                let frequencies_hz: Vec<u64> = (0..num_points)
                    .map(|i| (start_freq.as_hz() + i as u64 * step.as_hz()))
                    .collect();

                debug!(
                    "Received trace with {} data points: {}-{} Hz",
                    amplitudes_dbm.len(),
                    start_freq.as_hz(),
                    frequencies_hz.last().cloned().unwrap_or_default()
                );

                let trace_data = TraceData {
                    frequencies_hz,
                    amplitudes_dbm,
                };

                Ok(trace_data)
            }
            Err(e) => Err(WavewatchError::RfExplorer(e)),
        }
    }
}
