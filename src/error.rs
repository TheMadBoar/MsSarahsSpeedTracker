use std::string::FromUtf8Error;

#[allow(dead_code)]
pub type SpeedtestResult<T> = Result<T, SpeedtestError>;

#[derive(Debug)]
pub enum SpeedtestError {
    CLIError(String),
    JSONError(String),
    IOError(std::io::Error),
    UTF8Error(FromUtf8Error)
}

impl core::fmt::Display for SpeedtestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpeedtestError::CLIError(err) => write!(f, "{}", err),
            SpeedtestError::JSONError(err) => write!(f, "{}", err),
            SpeedtestError::IOError(err) => write!(f, "{}", err.to_string()),
            SpeedtestError::UTF8Error(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<serde_json::Error> for SpeedtestError {
    fn from(err: serde_json::Error) -> Self {
        SpeedtestError::JSONError(err.to_string())
    }
}

impl From<std::io::Error> for SpeedtestError {
    fn from(err: std::io::Error) -> Self {
        SpeedtestError::IOError(err)
    }
}

impl From<FromUtf8Error> for SpeedtestError {
    fn from(err: FromUtf8Error) -> Self {
        SpeedtestError::UTF8Error(err)
    }
}
