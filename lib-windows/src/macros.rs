#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        ::std::println!($($arg)*);
    }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        // Do nothing in release mode
    }
}

pub(crate) use debug_println;
