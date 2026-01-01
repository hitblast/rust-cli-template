use crate::cli::atomic::{should_be_quiet, should_be_verbose};

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const PINK: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

#[derive(PartialEq)]
#[doc(hidden)]
pub enum LogLevel {
    Error,
    Info,
    Prompt,
    Dry,
    Cute,
}

#[doc(hidden)]
pub fn _print_log(level: LogLevel, msg: &str) {
    if should_be_quiet() && level != LogLevel::Error {
        return;
    }

    if level == LogLevel::Info && !should_be_verbose() {
        return;
    }

    let (tag, color) = match level {
        LogLevel::Error => ("ERR", RED),
        LogLevel::Info => ("INF", CYAN),
        LogLevel::Prompt => ("ASK", PINK),
        LogLevel::Dry => ("DRY", YELLOW),
        LogLevel::Cute => ("🍎", ""),
    };

    let line = format!("{color}{tag}{RESET} {msg}");

    if level == LogLevel::Error {
        eprintln!("{line}");
    } else {
        println!("{line}");
    }
}

/// Macros.
#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Error, &msg);
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Info, &msg);
    }};
}

#[macro_export]
macro_rules! log_prompt {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Prompt, &msg);
    }};
}

#[macro_export]
macro_rules! log_dry {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Dry, &msg);
    }};
}

#[macro_export]
macro_rules! log_cute {
    ($($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::utils::logger::_print_log($crate::utils::logger::LogLevel::Cute, &msg);
    }};
}
