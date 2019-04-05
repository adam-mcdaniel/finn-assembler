mod strings;
mod tokenizer;
mod parser;
mod compile;
mod file;
mod stdout;
mod error;
mod check;

use file::*;
use error::*;
use stdout::*;
use compile::*;

use clap::*;


fn main() {
    let matches = clap_app!(finn_assembler =>
            (version: "0.2.2")
            (author: "Adam McDaniel <adam.mcdaniel17@gmail.com>")
            (about: "Assembles Finn IR into Rust using the HLVM as a backend")
            (@arg INCLUDED_CRATES: -c --crates +takes_value +multiple "Paths of crates to include")
            (@arg INPUT_FILE: +required "Input Finn IR filename")
            (@arg debug: -d --debug "Sets the level of debugging information")
        ).get_matches();

    let crates = match matches.values_of("INCLUDED_CRATES") {
        Some(a) => {
            let v: Vec<_> = a.map(|s| s.to_string()).collect();
            v
        },
        None => vec![]
    };

    let script_file_name = matches.value_of("INPUT_FILE").unwrap();
    let debug_level = matches.occurrences_of("debug");
    

    start();
    if debug_level > 0 {
        sub_debug(&format!("Included crates: {:?}", crates));
        sub_debug(&format!("Input file to assemble: {:?}", script_file_name));
        sub_debug(&format!("Debug level: {:?}", debug_level));
    }

    info("Reading input script");
    let script = read_file(script_file_name);

    if has_thrown_error() {
        error(
            &format!("Could not start compilation due to errors")
        );
        stop();
    } else {
        success("Successfully read input script");
    }

    write_deps(crates.clone());
    write_compiled_script(
        compile_script(crates.clone(), script)
        );

    compile_output_crate();

    done();
}