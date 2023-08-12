use chrono::{DateTime, Datelike, Local, Timelike};
use std::{fs::OpenOptions, io::Write, str};
use system::{del_dir, is_path, make_dir};

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


    return format!("{}-{}-{}-{}:{}.{}", year_string, month, day, hour, minute, second);
}

/// Creats a log directory and a file called general.log in the directory
pub fn start_log(prog: &str) -> Option<bool> {
    let log_msg: String = format!("LOG STARTED @{} \n", timestamp());

    // make the path
    let log_path = format!("{}{}", LOG_DIRECTORY, prog);


    del_dir(&log_path);
    if !make_dir(&log_path).unwrap() {
        return Some(false);
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
    let log_msg: String = format!("{} @{} \n", data, timestamp());

    // Opening the file

    let log_file: String = format!("{}{}/general.log", LOG_DIRECTORY, prog);

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
