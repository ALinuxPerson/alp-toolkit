#[macro_use]
pub mod log {
    use std::fmt;
    use owo_colors::{Color, OwoColorize};
    use owo_colors::colors::*;

    const PROLOGUE: char = '┃';
    const PROLOGUE_CONTINUATION: char = '=';

    fn log<C: Color, M: fmt::Display>(message: M) {
        let message = message.to_string();
        let mut lines = message.lines();
        let first_line = if let Some(first_line) = lines.next() {
            first_line
        } else {
            return
        };
        eprintln!("{} {first_line}", PROLOGUE.bold().fg::<C>());

        for line in lines {
            eprintln!("{} {line}", PROLOGUE_CONTINUATION.bold().fg::<C>());
        }
    }

    macro_rules! log_fn {
        ($($vis:vis $fn_name:ident, $color:ident;)*) => {
            $(
            $vis fn $fn_name(message: impl fmt::Display) {
                log::<$color, _>(message);
            }
            )*
        };
    }

    log_fn! {
        pub info, Blue;
        pub warn, Yellow;
        pub error, Red;
        pub tip, Green;
        pub debug, Cyan;
    }

    macro_rules! log {
        (
            some funny witty comment about the $d:tt token;
            $($fn_name:ident,)*
        ) => {
            $(
            #[macro_export]
            macro_rules! $fn_name {
                ($d($d arg:tt)*) => {
                    $crate::log::$fn_name(::std::format_args!($d($d arg)*));
                }
            }
            )*
        };
    }

    log! {
        some funny witty comment about the $ token;
        info, warn, error, tip, debug,
    }
}