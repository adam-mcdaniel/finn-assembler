use hlvm::function::*;

extern crate term;
use std::io::prelude::*;

use term::stdout;
use term::color::*;



fn print_color(s: String, fg: Option<Color>, bg: Option<Color>) {
    let mut t = stdout();

    match &mut t {
        Some(k) => {
            match fg {
                Some(fg) => match k.fg(fg) {_ => {}},
                None => {}
            }
            match bg {
                Some(bg) => match k.bg(bg) {_ => {}},
                None => {}
            }
            match write!(k, "{}", s) {
                Ok(_) => {},
                Err(_) => {}
            };
            match k.reset() {_ => {}};
        },
        None => {
            print!("{}", s);
        }
    };
}


pub fn print_red(s: Value) -> Value {
    print_color(
	s.as_string(),
        Some(BRIGHT_RED),
        None
    );
    none()
}

pub fn print_cyan(s: Value) -> Value {
    print_color(
        s.as_string(),
        Some(BRIGHT_CYAN),
        None
    );
    none()
}
