use crate::logger::log_enum::Log;
use std::{ffi::OsStr, fs::File, io::{Write, Read}, path::PathBuf};

/// This takes in a PathBuf
/// The paths will be looked at like different paths
/// This allows you to write the same log to multiple files
pub fn log(log: Log, file: PathBuf) {
    for file in file.iter() {
        let open_file: Result<File, std::io::Error> = File::open(file);

        let write_value = match log {
            Log::Error(err) => format!("  [ERROR] : {} \n", err),
            Log::Print(value) => format!("  [PRINT] : {} \n", value),
            Log::Warning(value) => format!(" [WARNING] : {} \n", value),
        };

        write_to_file(open_file, write_value, file);
    }
}

fn write_to_file(
    open_file: Result<File, std::io::Error>,
    write_value: String,
    fallback_filename: &OsStr,
) {
    let open_file = open_file;
    let mut file_text = String::new();
    if open_file.is_ok() {
          open_file.unwrap().read_to_string(&mut file_text).expect("couldn't write buffer to file_text");
    }
    file_text.push_str(&write_value);
    let err = format!("couldn't create file at: {:#?}", fallback_filename);
    let mut create_file = File::create(fallback_filename).expect(&err);
    let _ = create_file.write_all(file_text.as_bytes());
}
