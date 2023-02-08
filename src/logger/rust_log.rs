use std::{fs::File, io::Write};
use super::{log_enum::Log, log_struct::LogSet};

pub trait RustLog {
    fn log(&self, log: Log, file: &'static str, trace: Option<&'static str>);
}

pub trait WriteLog {
    fn write(&self, log: Log, log_file: &'static str, trace: Option<&'static str>)
    where
        Self: RustLog,
    {
        let file = if !trace.is_none() {
            let file = format!("{:?}\n{}", log, trace.unwrap());
            file
        } else {
            let file = format!("{:?}\n{}", log, trace.unwrap());
            file
        };
        let mut write = File::create(log_file).unwrap_or(File::open(log_file).unwrap());
        write.write_all(file.as_bytes()).expect("Couldn't write logs to file");
    }
}

impl WriteLog for LogSet{}


impl Default for LogSet {
    fn default() -> Self {
        LogSet { level: 0 }
    }
}


impl RustLog for LogSet {
    fn log(&self, log: Log, file: &'static str, trace: Option<&'static str>) {
        match self.level {
            0 => return,
            1 => WriteLog::write(self, log, file, trace),
            l => {
                println!("[RUST LOG] {} is not a log level", l);
            }
        }
    }
}
