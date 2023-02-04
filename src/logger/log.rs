use crate::logger::log_enum::Log;
use std::{ffi::OsStr, fs::File, io::{Write, Read}, path::PathBuf};
use chrono::Utc;

/// This takes in a PathBuf
/// The paths will be looked at like different paths
/// This allows you to write the same log to multiple files
pub fn log(log: Log, file: PathBuf, error_trace:Option<&'static str>) {
    for file in file.iter() {
        let open_file: Result<File, std::io::Error> = File::open(file);
        let time = Utc::now();
        let time = time.format("%Y-%m-%d %H:%M:%s").to_string();
        let line = line!();

        let write_value = match log {
            Log::Error(err) => format!("{} |  [ERROR] | [LINE] {} | {} \n", time, line, err),  
            Log::Print(value) => format!("{} |  [PRINT] | [LINE] {} | {} \n", time, line, value), 
            Log::Warning(value) => format!("{} |  [WARNING] | [LINE] {} | {} \n", time, line ,value), 
            Log::Info(value) => format!("{} | [INFO] | [LINE] {} | {} \n", time, line,value)
        };

        write_to_file(open_file, write_value, file, error_trace);
    }
}

fn write_to_file(
    open_file: Result<File, std::io::Error>,
    write_value: String,
    fallback_filename: &OsStr,
    error_trace:Option<&'static str>
) {
    let open_file = open_file;
    let mut file_text = String::new();
    if open_file.is_ok() {
          open_file.unwrap().read_to_string(&mut file_text).expect("couldn't write buffer to file_text");
    }
    file_text.push_str(&write_value);
    let err = format!("couldn't create file at: {:#?}", fallback_filename);
    let mut create_file = File::create(fallback_filename).expect(&err);
    let write = match error_trace {
        Some(trace) => format!("{} \n {} \n", file_text, trace),
        None => file_text
    };
    let _ = create_file.write_all(write.as_bytes()).expect("couldn't write to file");
}
