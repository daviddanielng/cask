macro_rules! exit_and_error {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31mError: {}\x1b[0m", format!($($arg)*));
        std::process::exit(1);
    }};
}
macro_rules! log_info {
    ($($arg:tt)*) => {{
        $crate::utils::logger::log_info(&format!($($arg)*));
    }};
}

macro_rules! log_error {
    ($($arg:tt)*) => {{
       eprintln!("\x1b[31mError: {}\x1b[0m", format!($($arg)*));
    }};
}
macro_rules! log_verbose {
    ($($arg:tt)*) => {{
            $crate::utils::logger::log_verbose(&format!($($arg)*));
        
    }};
}
macro_rules! log_warning {
    ($($arg:tt)*) => {{
            $crate::utils::logger::log_warning(&format!($($arg)*));
        
    }};
}

pub(crate) use exit_and_error;
pub(crate) use log_error;
pub(crate) use log_info;
pub(crate) use log_verbose;
pub(crate) use log_warning;
