/// Show debug info if devkit feature is enabled in compile time.
#[cfg(feature = "devkit")]
#[macro_export]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

/// Show debug info if devkit feature is enabled in compile time.
#[macro_export]
#[cfg(not(feature = "devkit"))]
macro_rules! dev_print {
    ($($arg:tt)*) => {};
}
