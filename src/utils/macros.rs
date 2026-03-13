macro_rules! exit_and_error {
    ($($arg:tt)*) => {{
        eprintln!("\x1b[31mError: {}\x1b[0m", format!($($arg)*));
        std::process::exit(1);
    }};
}

pub(crate) use exit_and_error;
