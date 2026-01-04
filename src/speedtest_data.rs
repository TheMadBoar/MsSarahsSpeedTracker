use std::{process::Command, time::{SystemTime, UNIX_EPOCH}};

use serde::Deserialize;

use crate::error::{SpeedtestError, SpeedtestResult};

pub fn run_speedtest() -> SpeedtestResult<SpeedtestStruct> {
    let mut exe_path = std::env::current_exe()?;
    exe_path.pop();
    exe_path.push("speedtest");
    let output = Command::new(exe_path)
        .arg("--accept-license")
        .arg("--format=json")
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SpeedtestError::CLIError(stderr.to_string()));
    }

    let stdout = String::from_utf8(output.stdout)?;

    let result: SpeedtestStruct = serde_json::from_str(&stdout)?;

    Ok(result)
}

pub async fn run_and_log_test() -> SpeedtestResult<SpeedtestStruct> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = now.as_secs();
    println!("[{}] Running speed test...", timestamp);
    run_speedtest()
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct SpeedtestStruct {
    #[serde(rename = "type")]
    pub test_type: String,
    pub timestamp: String,
    pub ping: Ping,
    pub download: Transfer,
    pub upload: Transfer,
    #[serde(rename = "packetLoss")]
    pub packet_loss: Option<u64>,
    pub isp: String,
    pub interface: Interface,
    pub server: Server,
    pub result: TestResult,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct TestResult {
    pub id: String,
    pub url: String,
    pub persisted: bool,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Ping {
    pub jitter: f64,
    pub latency: f64,
    pub low: f64,
    pub high: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Transfer {
    pub bandwidth: u64,
    pub bytes: u64,
    pub elapsed: u64,
    pub latency: Latency,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Latency {
    pub iqm: f64,
    pub low: f64,
    pub jitter: f64,
    pub high: f64,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interface {
    pub internal_ip: String,
    pub name: String,
    pub mac_addr: String,
    pub is_vpn: bool,
    pub external_ip: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Server {
    pub id: u64,
    pub host: String,
    pub port: u64,
    pub name: String,
    pub location: String,
    pub country: String,
    pub ip: String,
}
