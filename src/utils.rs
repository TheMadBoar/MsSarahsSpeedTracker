use std::{path::PathBuf, time::{SystemTime, UNIX_EPOCH}};

pub fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

pub fn get_app_data_dir<T>(program_name: T) -> std::io::Result<PathBuf>
where
    T: AsRef<str> + std::fmt::Display,
{
    let path = if cfg!(target_os = "windows") {
        let appdata = std::env::var("APPDATA")
            .map_err(|e| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to get APPDATA: {}", e)
            ))?;
        PathBuf::from(appdata).join(program_name.as_ref())
    } else if cfg!(target_os = "macos") {
        let home = std::env::var("HOME")
            .map_err(|e| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to get HOME: {}", e)
            ))?;
        PathBuf::from(home)
            .join("Library")
            .join("Application Support")
            .join(program_name.as_ref())
    } else {
        let home = std::env::var("HOME")
            .map_err(|e| std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Failed to get HOME: {}", e)
            ))?;
        PathBuf::from(home)
            .join(".local")
            .join("share")
            .join(program_name.as_ref())
    };
    std::fs::create_dir_all(&path)?;
    
    Ok(path)
}

#[allow(unused_macros)]
#[allow(dead_code)]
pub fn cur_utc_time() -> Result<(u16, u8, u8, u8, u8, u8, u16), std::io::Error> {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(since_the_epoch) => {
            let in_seconds = since_the_epoch.as_secs();

            // Calculate the number of days since the Unix Epoch
            let days_since_epoch = in_seconds / (24 * 3600);

            let hour = ((in_seconds / 3600) % 24) as u8;
            let minute = ((in_seconds / 60) % 60) as u8;
            let second = (in_seconds % 60) as u8;

            let millis = since_the_epoch.subsec_millis() as u16;
            // Calculate the current year, month, and day
            let (year, month, day) = get_date(days_since_epoch);

            Ok((year, month, day, hour, minute, second, millis))
        }
        Err(error) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid system time: {}", error),
        )),
    }
}

#[allow(dead_code)]
pub fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

#[allow(dead_code)]
pub fn get_date(mut days: u64) -> (u16, u8, u8) {
    let mut year = 1970;

    let days_in_year = |y| if is_leap_year(y) { 366 } else { 365 };

    while days >= days_in_year(year) {
        days -= days_in_year(year);
        year += 1;
    }

    let mut month = 1;
    let days_in_month: Vec<u8> = vec![
        0,
        31,
        if is_leap_year(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];

    while month <= 12 && days >= (days_in_month[month as usize] as u64) {
        days -= days_in_month[month as usize] as u64;
        month += 1;
    }

    (year, month, (days + 1) as u8)
}