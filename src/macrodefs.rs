/// Show debug info if devkit feature is enabled in compile time.
#[cfg(feature = "devkit")]
#[macro_export]
macro_rules! dev_print {
    ($($arg:tt)*) => {
        println!($($arg)*);
    };
}

/// Show debug info if devkit feature is enabled in compile time.
#[cfg(feature = "devkit")]
#[macro_export]
macro_rules! sender_dev_print {
    ($print_sender:expr, $($arg:tt)*) => {
        $print_sender.send(PrintMessage::DevPrint(format!($($arg)*).into())).unwrap_or_else(|e| eprintln!("Error sending message: {}", e));
    };
}

/// Show debug info if devkit feature is enabled in compile time.
#[cfg(not(feature = "devkit"))]
#[macro_export]
macro_rules! sender_dev_print {
    ($print_sender:expr, $($arg:tt)*) => {};
}

/// Show debug info if devkit feature is enabled in compile time.
#[macro_export]
#[cfg(not(feature = "devkit"))]
macro_rules! dev_print {
    ($($arg:tt)*) => {};
}
