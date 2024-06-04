#[deprecated(since = "0.2.1", note = "please use `unifiederrors` instead")]
pub mod errors;

use chrono::{DateTime, Datelike, Local, Timelike};
use std::{fs::OpenOptions, io::Write, str};
use system::{
    errors::{ErrorArray, ErrorArrayItem, UnifiedResult as uf},
    functions::{make_dir, open_file, remake_dir},
    types::PathType,
};

const LOG_DIRECTORY: &str = "/tmp/";

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

    fn _padding_date(number: u32) -> String {
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
        year_string, month, day, hour, minute, second
    );
}

/// Creats a log directory and a file called general.log in the directory
#[deprecated(
    since = "0.1.0",
    note = "Currently use `append_log()` with a valid progname, it will handel creating and managing log files"
)]
pub fn start_log(prog: &str, mut errors: ErrorArray) -> uf<()> {
    if prog == String::from("undefined") {
        errors.push(ErrorArrayItem::new(
            system::errors::Errors::GeneralError,
            String::from("Program name was invalid"),
        ));
        return uf::new(Err(errors));
    };

    let log_msg: String = format!("LOG STARTED @{} \n", timestamp());

    // make the path
    let log_path: PathType = PathType::Content(format!("{}{}", LOG_DIRECTORY, prog));

    // match remake_dir(&log_path.clone(), errors.clone()).uf_unwrap() {
    //     Ok(_) => (),
    //     Err(e) => return uf::new(Err(e)),
    // }

    let mut log_file: String = String::new();
    log_file.push_str(&log_path.as_os_str().to_string_lossy());
    log_file.push_str("/general.log");

    let mut log_file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(log_file)
    {
        Ok(d) => d,
        Err(e) => {
            errors.push(ErrorArrayItem::from(e));
            return uf::new(Err(errors));
        }
    };

    match writeln!(log_file, "{}", log_msg) {
        Ok(_) => uf::new(Ok(())),
        Err(e) => {
            errors.push(ErrorArrayItem::from(e));
            uf::new(Err(errors))
        }
    }
}

pub fn append_log(prog: &str, data: &str, mut errors: ErrorArray) -> uf<()> {
    // This function takes a str and program names wraps the data in a timestamp them write it to the appropriate file
    

    let log_msg: String = format!("{} @{} \n", data, timestamp());

    // Opening the file
    let log_dir: PathType = PathType::Content(format!("{}{}", LOG_DIRECTORY, prog));
    let log_file: PathType = PathType::Content(format!("{}/general.log", log_dir.to_string()));

    match remake_dir(&log_dir, errors.clone()).uf_unwrap() {
        Ok(_) => (),
        Err(e) => {
            println!("file err");
            return uf::new(Err(e))}
            ,
    }

    let mut log_file = match OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(log_file)
    {
        Ok(d) => d,
        Err(e) => {
            errors.push(ErrorArrayItem::from(e));
            return uf::new(Err(errors));
        }
    };

    // TODO USE THE SYSTEM LIB MORE HEAVY add append to file function to standardize behavior
    match writeln!(log_file, "{}", log_msg) {
        Ok(_) => return uf::new(Ok(())),
        Err(e) => {
            println!("err writing");
            errors.push(ErrorArrayItem::from(e));
            return uf::new(Err(errors));
        }
    };
}

#[cfg(test)]
mod tests {
    use system::functions::{del_dir, path_present};

    use super::*;

    #[test]
    fn logger() {
        let errors = ErrorArray::new_container();
        let path: PathType = PathType::Str("/tmp/test".into());
        let d = append_log("test", "Data", errors.clone());
        assert!(d.is_ok());
        // assert_eq!(
            // path_present(&path, errors.clone()).unwrap(),
            // true
        // );
        // Cleaning up dir
        let _ = del_dir(&path, errors.clone());
    }
}
