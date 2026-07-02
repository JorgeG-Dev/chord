//! Module handling all the UI related aspects of chord

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
        println!("{} {}", "warn:".red().bold(), format!($($arg)*));
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
