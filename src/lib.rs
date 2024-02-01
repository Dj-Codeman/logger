pub mod errors;

use chrono::{DateTime, Datelike, Local, Timelike};
use errors::{LoggerError, MyErrors};
use std::{fs::OpenOptions, io::Write, str};
use system::{errors::SystemError, remake_dir};

const LOG_DIRECTORY: &str = "/tmp/logger/";

fn timestamp() -> String {
    // Getting the data
    let current_time: DateTime<Local> = Local::now();

    let day: u32 = current_time.day();
    let month: u32 = current_time.month();
    let year: i32 = current_time.year();
    let hour: u32 = current_time.hour();
    let minute: u32 = current_time.minute();
    let second: u32 = current_time.second();

    // adding foward 0 padding to dates
    let year_string: String = year.to_string();

    fn padding_date(number: u32) -> String {
        if number < 10 {
            let mut local_date_string = String::new();
            local_date_string.push_str("0");
            local_date_string.push_str(&number.to_string());
            return local_date_string;
        } else {
            let local_date_string: String = String::from(&number.to_string());
            return local_date_string;
        }
    }

    return format!(
        "{}-{}-{}-{}:{}.{}",
        year_string,
        padding_date(month),
        padding_date(day),
        hour,
        minute,
        second
    );
}

/// Creats a log directory and a file called general.log in the directory
pub fn start_log(prog: &str) -> Result<(), MyErrors> {
    if prog == String::from("undefined") {
        return Err(MyErrors::LoggerError(LoggerError::new(errors::LoggerErrorType::InvalidProgName)));
    };

    let log_msg: String = format!("LOG STARTED @{} \n", timestamp());

    // make the path
    let log_path = format!("{}{}", LOG_DIRECTORY, prog);

    match remake_dir(&log_path) {
        Ok(_) => (), // TODO figure out a more compact way to handel this
        Err(e) => return Err(MyErrors::SystemError(e)),
    }

    let mut log_file: String = String::new();
    log_file.push_str(&log_path);
    log_file.push_str("/general.log");

    let mut log_file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(log_file)
    {
        Ok(d) => d,
        Err(e) => {
            let data: String = e.to_string();
            return Err(MyErrors::LoggerError(LoggerError::new_details(
                errors::LoggerErrorType::ErrorCreatingLog,
                &data,
            )));
        }
    };

    match writeln!(log_file, "{}", log_msg) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(MyErrors::SystemError(SystemError::new_details(
                system::errors::SystemErrorType::ErrorOpeningFile,
                &e.to_string(),
            )))
        }
    };
}

pub fn append_log(prog: &str, data: &str) -> Result<(), MyErrors> {
    // This function takes a str and program names wraps the data in a timestamp them write it to the appropriate file
    let log_msg: String = format!("{} @{} \n", data, timestamp());
    let log_file: String = format!("{}{}/general.log", LOG_DIRECTORY, prog);

    let mut log_file: std::fs::File =
        match OpenOptions::new().write(true).append(true).open(log_file) {
            Ok(d) => d,
            Err(e) => {
                return Err(MyErrors::SystemError(SystemError::new_details(
                    system::errors::SystemErrorType::ErrorOpeningFile,
                    &e.to_string(),
                )))
            }
        };

    // TODO USE THE SYSTEM LIB MORE HEAVY add append to file function to standardize behavior
    match writeln!(log_file, "{}", log_msg) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(MyErrors::SystemError(SystemError::new_details(
                system::errors::SystemErrorType::ErrorOpeningFile,
                &e.to_string(),
            )))
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use system::is_path;

    #[test]
    fn logger() {
        let _ = start_log("TEST").unwrap();
        assert_eq!(is_path("/tmp/logger/TEST"), true);
    }
}
