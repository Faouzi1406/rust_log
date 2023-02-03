#[cfg(test)] 
mod test {
    use std::path::PathBuf;

    use crate::logger::log::log;
    #[test]
    fn test_error_log() {
        log(crate::logger::log_enum::Log::Error("This is a funy error message "), PathBuf::from("error.log"), None);
        let file_read = std::fs::read_to_string(PathBuf::from("error.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[ERROR]");
        //delete file 
        std::fs::remove_file(PathBuf::from("error.log")).unwrap();
    }

    #[test]
    fn test_print_log() {
        log(crate::logger::log_enum::Log::Print("This is a funy print message "), PathBuf::from("print.log"), None);
        let file_read = std::fs::read_to_string(PathBuf::from("print.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[PRINT]");
        //delete file 
        std::fs::remove_file(PathBuf::from("print.log")).unwrap();
    }

    #[test]
    fn test_warning_log() {
        log(crate::logger::log_enum::Log::Warning("This is a funy warning message "), PathBuf::from("warning.log"), None);
        let file_read = std::fs::read_to_string(PathBuf::from("warning.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[WARNING]");
        //delete file 
        std::fs::remove_file(PathBuf::from("warning.log")).unwrap();
    }

    #[test]
    fn test_warning_info() {
        log(crate::logger::log_enum::Log::Info("This is a funy warning message "), PathBuf::from("info.log"), None);
        let file_read = std::fs::read_to_string(PathBuf::from("info.log")).unwrap();
        let file:Vec<&str> = file_read.split("|").collect();
        assert_eq!(file[1].trim(), "[INFO]");
        //delete file 
        std::fs::remove_file(PathBuf::from("info.log")).unwrap();
    }
}
