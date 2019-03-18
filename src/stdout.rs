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


pub fn title() {
    print_color(
        "     dBBBBP dBP dBBBBb  dBBBBb     dBBBBBb  .dBBBBP   dBBBBBBb
                   dBP     dBP          BB  BP             dBP
   dBBBP  dBP dBP dBP dBP dBP       dBP BB  `BBBBb  dBPdBPdBP 
  dBP    dBP dBP dBP dBP dBP       dBP  BB     dBP dBPdBPdBP  
 dBP    dBP dBP dBP dBP dBP       dBBBBBBBdBBBBP' dBPdBPdBP   ".to_string(),
        Some(WHITE),
        None
    );
    println!("");
}

pub fn start() {
    // title();
    print!("===[ START ]===> ");
    print_color(
        "Assembling".to_string(),
        Some(MAGENTA),
        None
    );
    println!("");
}

pub fn success(s: &str) {
    print!("\t   └─> ");
    // print!("==| SUCCESS |==> ");
    print_color(
        format!("{}", s),
        Some(GREEN),
        None
    );
    println!("");
}

pub fn done() {
    print!("===[ DONE ]====> ");
    print_color(
        "Finished Assembling".to_string(),
        Some(MAGENTA),
        None
    );
    println!("");
}



#[allow(dead_code)]
pub fn warn(s: &str) {
    print!("===[ WARN ]====> ");
    print_color(
        format!("{}", s),
        Some(YELLOW),
        None
    );
    println!("");
}

#[allow(dead_code)]
pub fn sub_warn(s: &str) {
    print!("\t   └─> ");
    print_color(
        format!("{}", s),
        Some(YELLOW),
        None
    );
    println!("");
}


pub fn error(s: &str) {
    print!("===[ ERROR ]===> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_RED),
        None
    );
    println!("");
}

pub fn sub_error(s: &str) {
    print!("\t   └─> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_RED),
        None
    );
    println!("");
}

pub fn sub_error_info(s: &str) {
    print!("\t\t   └─> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_RED),
        None
    );
    println!("");
}


pub fn info(s: &str) {
    // print!("===( INFO )====> ");
    print!("==> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_BLUE),
        None
    );
    println!("");
}

pub fn sub_info(s: &str) {
    print!("\t   └─> ");
    print_color(
        format!("{}", s),
        Some(CYAN),
        None
    );
    println!("");
}


#[allow(dead_code)]
pub fn debug(s: &str) {
    print!("==={{ DEBUG }}===> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_YELLOW),
        None
    );
    println!("");
}

pub fn sub_debug(s: &str) {
    print!("\t   └─> ");
    print_color(
        format!("{}", s),
        Some(BRIGHT_YELLOW),
        None
    );
    println!("");
}
