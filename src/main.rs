use std::{
    fs::OpenOptions,
    io::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tokio::{signal, sync::mpsc, time::interval};

use crate::{
    speedtest_data::{SpeedtestStruct, run_and_log_test},
    utils::{cur_utc_time, escape_csv_field, get_app_data_dir},
};

mod error;
mod speedtest_data;
mod utils;

static RUN_INTERVAL: u64 = 300;
const APP_NAME: &'static str = "send comic feet pics";

#[tokio::main]
async fn main() {
    println!("To exit this Speedtest CLI Data logger: press Ctrl+C");
    let (tx, mut rx) = mpsc::channel::<SpeedtestStruct>(32);

    match get_app_data_dir(APP_NAME) {
        Ok(app_directory) => {
            let mut output_log_file = app_directory.clone();
            output_log_file.push("output_logs");
            match std::fs::create_dir_all(&output_log_file) {
                Ok(()) => {
                    let (year, month, day, hour, minute, second, _ms) = match cur_utc_time() {
                        Ok(data) => data,
                        Err(_) => (0, 0, 0, 0, 0, 0, 0),
                    };
                    let file_name = format!(
                        "{:04}-{:02}-{:02} {:02}-{:02}-{:02}.csv",
                        year, month, day, hour, minute, second
                    );
                    output_log_file.push(file_name);
                    match OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(&output_log_file)
                    {
                        Ok(mut file) => {
                            match writeln!(
                                file,
                                "Timestamp,Latency,Download (Mbps),Upload (Mbps),ISP,Server Location,Result URL"
                            ) {
                                Ok(()) => {
                                    let output_log_file_clone = output_log_file.clone();
                                    let receiver_handle = tokio::spawn(async move {
                                        let mut upload_speeds_vec = Vec::<f64>::new();
                                        let mut download_speeds_vec = Vec::<f64>::new();
                                        loop {
                                            tokio::select! {
                                                cli_result = rx.recv() => {
                                                    match cli_result {
                                                        Some(res) => {
                                                            let upload_speed = (res.upload.bandwidth as f64 * 8.0) / 1_000_000.0;
                                                            upload_speeds_vec.push(upload_speed);
                                                            let download_speed = (res.download.bandwidth as f64 * 8.0) / 1_000_000.0;
                                                            download_speeds_vec.push(download_speed);
                                                            if output_log_file_clone.exists() {
                                                                if let Err(err) = writeln!(
                                                                    file,
                                                                    "{},{},{},{},{},{},{}",
                                                                    escape_csv_field(&res.timestamp),
                                                                    res.ping.latency,
                                                                    download_speed,
                                                                    upload_speed,
                                                                    escape_csv_field(&res.isp),
                                                                    escape_csv_field(&res.server.location),
                                                                    escape_csv_field(&res.result.url)
                                                                ) {
                                                                    println!("Error writing to the output file: {}", err);
                                                                };
                                                            } else {
                                                                println!("Log file no longer exists, exiting.");
                                                                break;
                                                            }
                                                        },
                                                        None => {
                                                            println!("CLI Transmitter has closed");
                                                            break;
                                                        }
                                                    }
                                                }
                                                _ = signal::ctrl_c() => {
                                                    println!("\nShutting down gracefully...");
                                                    let mut total_download: f64 = 0.0;
                                                    let amount_of_download_checks = download_speeds_vec.len();
                                                    for download in download_speeds_vec {
                                                        total_download = total_download + download;
                                                    }
                                                    let average_download = total_download / amount_of_download_checks as f64;
                                                    println!("Average Download Speed: {}", average_download);
                                                    let mut total_upload: f64 = 0.0;
                                                    let amount_of_upload_checks = upload_speeds_vec.len();
                                                    for upload in upload_speeds_vec {
                                                        total_upload = total_upload + upload;
                                                    }
                                                    let average_upload = total_upload / amount_of_upload_checks as f64;
                                                    println!("Average Upload Speed: {}", average_upload);

                                                    if let Err(err) = writeln!(
                                                        file,
                                                        "\r\nAverage Download Speed(in Mbps),Average Upload Speed(in Mbps)"
                                                    ) {
                                                        println!("Error writing to the output file: {}", err);
                                                    };
                                                    if let Err(err) = writeln!(
                                                        file,
                                                        "{},{}",
                                                        average_download,
                                                        average_upload,
                                                    ) {
                                                        println!("Error writing to the output file: {}", err);
                                                    };
                                                    println!("Your .csv file has been saved to:");
                                                    println!("{}", output_log_file.to_string_lossy().to_string());
                                                    break;
                                                }
                                            }
                                        }
                                    });
                                    tokio::spawn(async move {
                                        let mut interval =
                                            interval(Duration::from_secs(RUN_INTERVAL));
                                        interval.tick().await;
                                        loop {
                                            let tx_clone = tx.clone();
                                            tokio::spawn(async move {
                                                match run_and_log_test().await {
                                                    Ok(data) => {
                                                        let now = SystemTime::now()
                                                            .duration_since(UNIX_EPOCH)
                                                            .unwrap();

                                                        let timestamp = now.as_secs();
                                                        println!(
                                                            "[{}] Successfully ran a speedtest. You may quit at any time with Ctrl+C.",
                                                            &timestamp
                                                        );
                                                        tx_clone.send(data).await.unwrap();
                                                    }
                                                    Err(err) => {
                                                        let now = SystemTime::now()
                                                            .duration_since(UNIX_EPOCH)
                                                            .unwrap();

                                                        let timestamp = now.as_secs();
                                                        println!(
                                                            "[{}] Speedtest has failed: {}",
                                                            &timestamp, err
                                                        );
                                                    }
                                                };
                                            });
                                            interval.tick().await;
                                        }
                                    });

                                    match receiver_handle.await {
                                        Ok(()) => {}
                                        Err(err) => {
                                            println!(
                                                "Unable to get joinhandle of receiver handler: {}",
                                                err
                                            );
                                        }
                                    };
                                }
                                Err(err) => {
                                    println!("Error writing to the output file: {}", err);
                                }
                            }; // header
                        }
                        Err(err) => {
                            println!("Error opening the output file: {}", err);
                        }
                    };
                }
                Err(err) => {
                    println!("Error creating the output log directory: {}", err);
                }
            };
        }
        Err(err) => {
            println!("Error getting the output log directory: {}", err);
        }
    };
}
