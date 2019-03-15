use hlvm::function::*;


use std::io;
use std::io::Write;
pub fn get_input(prompt: &str) -> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}


pub fn input(v: Value) -> Value {
    string(&get_input(&v.as_string()))
}