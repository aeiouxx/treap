use std::time::SystemTime;

pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
pub trait Logger {
    fn log(&self, level: LogLevel, message: &str);
    fn trace(&self, message: &str) {
        self.log(LogLevel::Trace, message);
    }
    fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
pub struct StdOutLogger;
impl Logger for StdOutLogger {
    fn log(&self, level: LogLevel, message: &str) {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let (color_code, level_str) = match level {
            LogLevel::Trace => ("\x1b[94m", "TRACE"),
            LogLevel::Debug => ("\x1b[36m", "DEBUG"),
            LogLevel::Info => ("\x1b[32m", "INFO "),
            LogLevel::Warn => ("\x1b[33m", "WARN "),
            LogLevel::Error => ("\x1b[31m", "ERROR"),
        };
        println!(
            "{}{} [{:?}] {}\x1b[0m",
            color_code, level_str, since_the_epoch, message
        )
    }
}
