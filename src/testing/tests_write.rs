#[cfg(test)] 
mod test {
    use std::path::PathBuf;

    use crate::logger::{log::log, log_enum::LogLevel};
    #[test]
    fn test_error_log() {
        log(crate::logger::log_enum::Log::Error("This is a funny error message "), PathBuf::from("error.log"), None, LogLevel::LevelError);
        let file_read = std::fs::read_to_string(PathBuf::from("error.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[ERROR]");
        //delete file 
        std::fs::remove_file(PathBuf::from("error.log")).unwrap();
    }

    #[test]
    fn test_print_log() {
        log(crate::logger::log_enum::Log::Print("This is a funny print message "), PathBuf::from("print.log"), None, LogLevel::LevelPrint);
        let file_read = std::fs::read_to_string(PathBuf::from("print.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[PRINT]");
        //delete file 
        std::fs::remove_file(PathBuf::from("print.log")).unwrap();
    }

    #[test]
    fn test_warning_log() {
        log(crate::logger::log_enum::Log::Warning("This is a funny warning message "), PathBuf::from("warning.log"), None, LogLevel::LevelWarning);
        let file_read = std::fs::read_to_string(PathBuf::from("warning.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[WARNING]");
        //delete file 
        std::fs::remove_file(PathBuf::from("warning.log")).unwrap();
    }

    #[test]
    fn test_warning_info() {
        log(crate::logger::log_enum::Log::Info("This is a funny warning message "), PathBuf::from("info.log"), None, LogLevel::All);
        let file_read = std::fs::read_to_string(PathBuf::from("info.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[INFO]");
        //delete file 
        std::fs::remove_file(PathBuf::from("info.log")).unwrap();
    }

    #[test]
    fn test_log_message_print() {
        log(crate::logger::log_enum::Log::Print("This is a funny print message"), PathBuf::from("print_message.log"), None, LogLevel::All);
        let file_read = std::fs::read_to_string(PathBuf::from("print_message.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[4].trim(), "This is a funny print message");
        //delete file 
        std::fs::remove_file(PathBuf::from("print_message.log")).unwrap();
    } 
    
    #[test]
    fn test_log_message_error() {
        log(crate::logger::log_enum::Log::Error("This is a funny error message"), PathBuf::from("error_message.log"), None, LogLevel::All);
        let file_read = std::fs::read_to_string(PathBuf::from("error_message.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[4].trim(), "This is a funny error message");
        //delete file 
        std::fs::remove_file(PathBuf::from("error_message.log")).unwrap();
    }

    #[test]
    fn test_log_message_warning() {
        log(crate::logger::log_enum::Log::Warning("This is a funny warning message"), PathBuf::from("warning_message.log"), None, LogLevel::All);
        let file_read = std::fs::read_to_string(PathBuf::from("warning_message.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[4].trim(), "This is a funny warning message");
        //delete file 
        std::fs::remove_file(PathBuf::from("warning_message.log")).unwrap();
    }

    #[test]
    fn test_file_log(){
        log(crate::logger::log_enum::Log::Print("This is a funny print message"), PathBuf::from("print_file.log"), None, LogLevel::All);
        let file_read = std::fs::read_to_string(PathBuf::from("print_file.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[4].trim(), "This is a funny print message");
        //delete file 
        std::fs::remove_file(PathBuf::from("print_file.log")).unwrap();
    }
}
