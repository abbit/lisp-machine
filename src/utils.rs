#[macro_export]
macro_rules! debug {
    // Match the pattern for debug builds
    ($($arg:tt)*) => {
        // Check if the 'debug_assertions' feature is enabled
        #[cfg(debug_assertions)]
        {
            // If debug build, use println!
            println!($($arg)*);
        }
        // Otherwise, do nothing
    };
}
