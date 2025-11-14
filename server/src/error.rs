use colored::*;

#[derive(PartialEq)]
pub enum LogType {
    Info,
    Warn,
    Error,
}

pub fn logger(log_type: LogType, log: &str) {
    match log_type {
        LogType::Info => {
            println!("{}: {}", "INFO".blue(), log);
        }
        LogType::Warn => {
            println!("{}: {}", "WARN".yellow(), log);
        }
        LogType::Error => {
            println!("{}: {}", "ERROR".red(), log);
        }
    }
}
