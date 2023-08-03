use chrono::{DateTime, Datelike, Local, Timelike};
use std::{fs::OpenOptions, io::Write, str};
use system::{deldir, ispath, makedir};

const LOG_DIRECTORY: &str = "/tmp/logger/";

fn timestamp() -> String {
    // Getting the data
    let mut timestamp: String = String::new();
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

    timestamp.push_str(&year_string);
    timestamp.push_str("-");
    timestamp.push_str(&padding_date(month));
    timestamp.push_str("-");
    timestamp.push_str(&padding_date(day));
    timestamp.push_str("_");
    timestamp.push_str(&padding_date(hour));
    timestamp.push_str(":");
    timestamp.push_str(&padding_date(minute));
    timestamp.push_str(".");
    timestamp.push_str(&padding_date(second));

    return timestamp;
}

/// Creats a log directory and a file called general.log in the directory
pub fn start_log(prog: &str) -> Option<bool> {
    let mut log_msg: String = String::new();
    log_msg.push_str(" LOG START");
    log_msg.push_str(" @ ");
    log_msg.push_str(&timestamp());
    log_msg.push_str("\n");

    // make the path
    let mut log_path: String = String::new();
    log_path.push_str(LOG_DIRECTORY);
    log_path.push_str(prog);

    if !ispath(&log_path) {
        if !makedir(&log_path).unwrap() {
            return Some(false);
        }
        // keeps going if the path is made
    } else {
        deldir(&log_path);
        if !makedir(&log_path).unwrap() {
            return Some(false);
        }
    }

    // checks if the path exists using the ispath function.
    let mut log_file: String = String::new();
    log_file.push_str(&log_path);
    log_file.push_str("/general.log");

    let mut log_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(log_file)
        .expect("File could not be opened");

    if let Err(_e) = writeln!(log_file, "{}", log_msg) {
        eprintln!("Could not create or write to new log file");
        return Some(false);
    }

    return Some(true);
}

pub fn append_log(prog: &str, data: &str) -> Option<bool> {
    // Makign data
    let mut log_msg: String = String::new();
    log_msg.push_str(data);
    log_msg.push_str(" @ ");
    log_msg.push_str(&timestamp());
    log_msg.push_str("\n");

    // Opening the file
    let mut log_file: String = String::new();
    log_file.push_str(LOG_DIRECTORY);
    log_file.push_str(prog);
    log_file.push_str("/general.log");

    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(log_file)
        .expect("File could not be opened");

    // Hendeling errs
    if let Err(_e) = writeln!(log_file, "{}", log_msg) {
        eprintln!("Couldn't open already existing log file");
        return Some(false);
    }; 

    return Some(true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger() {
        let result = start_log("TEST").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn append() {
        start_log("TEST").unwrap();
        let result = append_log("TEST", "data").unwrap();
        assert_eq!(result, true);
    }
}
