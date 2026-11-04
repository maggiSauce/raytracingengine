#[macro_export]
/* Usage:
 *   printdebug!("Message") => [DEBUG] Message
 *   printdebug!("Message", Some("TYPE")) => [TYPE] Message
 */
macro_rules! printdebug {
    ($message:expr) => {
        printdebug!($message, None);
    };

    ($message: expr, $debug_type: expr) => {
        #[cfg(feature = "debug")]
        {
            let debug_type = $debug_type.unwrap_or("DEBUG");
            println!("[{}] {}", debug_type, $message);
        }
    };
}
