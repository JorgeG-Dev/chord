//! Module handling all the UI related aspects of chord
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Indicator used to show the user that there's operations being performed on
/// the repo.
pub struct RepoProgress {
    bar: ProgressBar,
    name: String,
}

impl RepoProgress {
    /// Creates an instance of the RepoProgress struct
    pub fn new(name: impl AsRef<str>) -> Self {
        // This shouldn't ever fail
        let style = ProgressStyle::with_template("{spinner} {msg}").unwrap();
        let bar = ProgressBar::new_spinner();
        bar.set_style(style);
        bar.enable_steady_tick(Duration::from_millis(100));
        Self {
            name: String::from(name.as_ref()),
            bar,
        }
    }

    /// Meant to be used in any multi step repo processes
    pub fn step(&self, action: impl AsRef<str>) {
        self.bar
            .set_message(format!("{}: {}", self.name, action.as_ref()));
    }

    /// Meant to be used upon successful completion of repo operations
    pub fn done(&self) {
        self.bar
            .finish_with_message(format!("{}: {}", self.name, "done".green()))
    }

    /// Meant to be used if the repo operations failed
    pub fn failed(&self, err: impl AsRef<str>) {
        self.bar.finish_with_message(format!(
            "{}: {} ({})",
            self.name,
            "failed".red(),
            err.as_ref()
        ));
    }
}

#[macro_export]
macro_rules! info_msg {
    ($($arg:tt)*) => {
        println!("{} {}", "info:".blue().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error_msg{
    ($($arg:tt)*) => {
        println!("{} {}", "error:".red().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn_msg{
    ($($arg:tt)*) => {
        println!("{} {}", "warn:".yellow().bold(), format!($($arg)*));
    };
}

#[macro_export]
macro_rules! repo_header {
    ($repo: expr) => {
        println!(
            "{}",
            format!("============ {} ============", $repo).blue().bold()
        );
    };
}
