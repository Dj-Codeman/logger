use std::fmt;
use system::errors::SystemError;

#[derive(Debug)]
pub enum MyErrors {
    LoggerError(LoggerError),
    SystemError(SystemError),
}

#[derive(Debug)]
pub struct LoggerError {
    pub kind: LoggerErrorType,
    pub details: Option<String>
}

#[derive(Debug)]
pub enum LoggerErrorType {
    InvalidProgName,
    ErrorCreatingDir,
    ErrorCreatingLog,
}

// pretty display
impl fmt::Display for LoggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.details {
            Some(d) => write!(f, "Logger Error: {} - {}", self.kind_description(), d),
            None => write!(f, "Logger Error: {}", self.kind_description()),
        }
    }
}

impl LoggerError {
    pub fn new(kind: LoggerErrorType) -> Self {
        LoggerError {
            kind, 
            details: None,
        }
    }

    pub fn new_details(kind: LoggerErrorType, details: &str) -> Self {
        LoggerError {
            kind,
            details: Some(details.to_string())
        }
    }

    fn kind_description(&self) -> String {
        match &self.kind {
            LoggerErrorType::InvalidProgName => String::from("Program name defined is invalid"),
            LoggerErrorType::ErrorCreatingDir => String::from("An error occoured while creating the directory for the logs"),
            LoggerErrorType::ErrorCreatingLog => String::from("An error occoured while creating the logfile"),
        }
    }
}