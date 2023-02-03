## 📝 Logger

This crate provides a simple logging solution for your Rust projects. It allows you to log messages with different levels of importance: `Error`, `Print`, `Warning`, and `Info`.

## 💻 Usage

First, include the following in your `Cargo.toml` file:

```Rust
logger = { git = "https://github.com/Faouzi1406/rust_log" }
```

Then, in your Rust file, use the following to import the crate:

```rust
use logger::log;
use logger::log_enum::Log;
```

To log a message, use the following format:

```Rust 
log(Log::Error("This is a sample error message"), PathBuf::from("error.log"), None);
```

Replace Error with the desired log level, and replace the message and log file path as needed.

# 🧪 Tests
The crate comes with tests for each log level to ensure proper functionality. To run the tests, simply use cargo test.
