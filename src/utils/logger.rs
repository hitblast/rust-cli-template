use crate::cli::atomic::{should_be_quiet, should_be_verbose};

/// ANSI color codes
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const PINK: &str = "\x1b[35m";
pub const ORANGE: &str = "\x1b[38;5;208m";
pub const CYAN: &str = "\x1b[36m";
pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

/// Log level determiner struct for print_log().
#[derive(PartialEq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Prompt,
    CommandOutput,
    Dry,
    Fruitful, // 🍎
}

/// Central logger.
pub fn print_log(level: LogLevel, msg: &str) {
    if should_be_quiet() && level != LogLevel::Error && level != LogLevel::Warning {
        return;
    }

    if (level == LogLevel::Info || level == LogLevel::CommandOutput) && !should_be_verbose() {
        return;
    }

    let (tag, color) = match level {
        LogLevel::Error => ("ERR  ", RED),
        LogLevel::Warning => ("WARN ", ORANGE),
        LogLevel::Info => ("INFO ", CYAN),
        LogLevel::CommandOutput => ("OUT  ", GREEN),
        LogLevel::Prompt => ("PRMT ", PINK),
        LogLevel::Dry => ("DRY  ", YELLOW),
        LogLevel::Fruitful => ("🍎", ""),
    };

    let line = if level == LogLevel::Fruitful {
        format!("{tag} {msg}")
    } else {
        format!("{color}{tag}{RESET} {msg}")
    };

    if level == LogLevel::Error || level == LogLevel::Warning {
        eprintln!("{line}");
    } else {
        println!("{line}");
    }
}
