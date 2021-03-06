#[macro_export]
macro_rules! cli_message {
        () => {
            {
                use std::io::Write;
                print!("\r{}", "wallet713> ");
                std::io::stdout().flush().unwrap();
            }
        };

        ($fmt_string:expr, $( $arg:expr ),+) => {
            {
                use std::io::Write;
                print!("\r");
                print!($fmt_string, $( $arg ),*);
                print!("\n{}", "wallet713> ");
                std::io::stdout().flush().unwrap();
            }
        };

        ($fmt_string:expr) => {
            {
                use std::io::Write;
                print!("\r");
                print!($fmt_string);
                print!("\n{}", "wallet713> ");
                std::io::stdout().flush().unwrap();
            }
        };
    }
