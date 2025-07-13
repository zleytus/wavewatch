use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "wavewatch-remote")]
#[command(about = "RF Explorer data collector for Wavewatch")]
pub struct Config {
    /// Server URL to send trace data to
    #[arg(
        long,
        default_value = "https://api.wavewatch.io",
        help = "Base URL of the Wavewatch server"
    )]
    pub server_url: String,

    /// RF Explorer device port/path
    #[arg(
        long,
        help = "Serial port path for RF Explorer (e.g., /dev/ttyUSB0, COM3)"
    )]
    pub device_port: Option<String>,

    /// Start frequency in Hz
    #[arg(long, help = "Start frequency for spectrum analysis in Hz")]
    pub start_freq: Option<u64>,

    /// Stop frequency in Hz
    #[arg(long, help = "Stop frequency for spectrum analysis in Hz")]
    pub stop_freq: Option<u64>,

    /// Enable verbose logging
    #[arg(short, long, help = "Enable verbose logging output")]
    pub verbose: bool,
}
