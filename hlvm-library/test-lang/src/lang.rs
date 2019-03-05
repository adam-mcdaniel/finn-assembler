extern crate hlvm;
use hlvm::function::{Fun, ForeignFunction, Value, Object, num, string, list};


pub fn interpret(script: &str) -> Fun {
    let mut result = Fun::new();
    let mut token = "".to_string();
    let mut is_inside_string = false;
    for ch in script.chars() {
        match ch {
            '"' => {
                if is_inside_string {
                    result.add_str(token.as_str());
                    token = "".to_string();
                    is_inside_string = false;
                } else {
                    is_inside_string = true;
                }
            }

            '!' => {
                if !is_inside_string {
                    result.call_from_stack();
                } else {
                    token += "!";
                }
            }

            c => {
                token += c.to_string().as_str();
            }
        }
    }

    return result.disassemble().println();
}