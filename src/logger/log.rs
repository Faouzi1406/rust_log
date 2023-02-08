use crate::logger::log_enum::Log;
use chrono::Utc;
use std::{
    ffi::OsStr,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use super::log_enum::LogLevel;

/// This takes in a PathBuf
/// The paths will be looked at like different paths
/// This allows you to write the same log to multiple files
#[track_caller]
pub fn log(log: Log, file: PathBuf, error_trace: Option<&'static str>, log_level: LogLevel) {
    for file in file.iter() {
        let open_file: Result<File, std::io::Error> = File::open(file);
        let time = Utc::now();
        let time = time.format("%Y-%m-%d %H:%M:%s").to_string();
        let caller = std::panic::Location::caller();
        let line = caller.line();
        let file_caller = caller.file();
        let mut type_error: i32 = 0;

        let write_value = match log {
            Log::Error(err) => {
                type_error = 3;
                format!(
                    "{} |  [ERROR] | [LINE] {} | [FILE] {} | {} \n",
                    time, line, file_caller, err
                )
            }
            Log::Print(value) => {
                type_error = 1;
                format!(
                    "{} |  [PRINT] | [LINE] {} | [FILE] {} | {} \n",
                    time, line, file_caller, value
                )
            }
            Log::Warning(value) => {
                type_error = 2;
                format!(
                    "{} |  [WARNING] | [LINE] {} | [FILE] {} | {} \n",
                    time, line, file_caller, value
                )
            }
            Log::Info(value) => {
                format!(
                    "{} | [INFO] | [LINE] {} | [FILE] {}  | {} \n",
                    time, line, file_caller, value
                )
            }
        };

        match log_level {
            LogLevel::All => write_to_file(open_file, write_value, file, error_trace),
            LogLevel::LevelError => {
                if type_error == 3 {
                    write_to_file(open_file, write_value, file, error_trace)
                }
            }
            LogLevel::LevelPrint => {
                if type_error == 3 || type_error == 2 || type_error == 1 {
                    write_to_file(open_file, write_value, file, error_trace)
                }
            }
            LogLevel::LevelWarning => {
                if type_error == 3 || type_error == 2  {
                    write_to_file(open_file, write_value, file, error_trace)
                }
            }
        }
    }
}

pub fn write_to_file(
    open_file: Result<File, std::io::Error>,
    write_value: String,
    fallback_filename: &OsStr,
    error_trace: Option<&'static str>,
) {
    let open_file = open_file;
    let mut file_text = String::new();
    if open_file.is_ok() {
        open_file
            .unwrap()
            .read_to_string(&mut file_text)
            .expect("couldn't write buffer to file_text");
    }
    file_text.push_str(&write_value);
    let err = format!("couldn't create file at: {:#?}", fallback_filename);
    let mut create_file = File::create(fallback_filename).expect(&err);
    let write = match error_trace {
        Some(trace) => format!("{} \n {} \n", file_text, trace),
        None => file_text,
    };
    let _ = create_file
        .write_all(write.as_bytes())
        .expect("couldn't write to file");
}
