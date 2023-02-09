use std::fmt::Write;
use std::{
    fmt::{self, Debug, Formatter},
};

use chrono::Utc;
pub enum Log {
    Error(&'static str),
    Warning(&'static str),
    Print(&'static str),
    Info(&'static str),
}

pub enum LogLevel {
    All,
    LevelWarning,
    LevelError,
    LevelPrint,
}

impl Debug for Log {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let time = Utc::now();
        let time = time.format("%Y-%m-%d %H:%M:%s").to_string();
        let caller = std::panic::Location::caller();
        let line = caller.line();
        let file_caller = caller.file();
        let mut format_value = String::new();

        let _ = match &self {
            Log::Error(err) => {
                write!(
                    &mut format_value,
                    "{} |  [ERROR] | [LINE] {} | [FILE] {} | {} \n",
                    time, line, file_caller, err
                )
            }
            Log::Print(value) => {
                write!(
                    &mut format_value,
                    "{} |  [PRINT] | [LINE] {} | [FILE] {} | {} \n",
                    time, line, file_caller, value
                )
            }
            Log::Warning(value) => {
                write!(
                    &mut format_value,
                    "{} |  [WARNING] | [LINE] {} | [FILE] {} | {} \n",
                    time,
                    line,
                    file_caller,
                    value
                )
            }
            Log::Info(value) => {
                write!(
                    &mut format_value,
                    "{} | [INFO] | [LINE] {} | [FILE] {}  | {} \n", time, line, file_caller, value
                )
            }
        };
        
        f.write_str(&format_value)
    }
}
