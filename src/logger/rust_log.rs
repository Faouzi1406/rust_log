use std::{fs::{File, read}, io::Write};
use super::{log_enum::Log, log_struct::LogSet};

pub trait RustLog {
    fn log(&self, log: Log, file: &'static str, trace: Option<String>);
}

trait WriteLog {
    fn write(&self, log: Log, log_file: &'static str, trace: Option<String>)
    where
        Self: RustLog,
    {
        let file = if trace.is_some() {
            println!("log: {:?}", log);
            let file = format!("{:?}\n{}", log, trace.unwrap());
            file
        } else {
            let file = format!("{:?}\n", log);
            file
        };
        let mut file_read = read(log_file).unwrap_or(vec![]);
        let mut write = File::create(log_file).expect("Coulnd't create file");
        for read in file.as_bytes() {
            file_read.push(*read);
        }
        write.write_all(&file_read.clone()).expect("Couldn't write to file");
    }
}

impl WriteLog for LogSet{}


impl Default for LogSet {
    /// Sets log level to default value of 0 
    fn default() -> Self {
        LogSet { level: 0 }
    }
}


impl RustLog for LogSet {
    fn log(&self, log: Log, file: &'static str, trace: Option<String>) {
        match self.level {
            0 => return,
            1 => WriteLog::write(self, log, file, trace),
            l => {
                println!("[RUST LOG] {} is not a log level", l);
            }
        }
    }
}
