use crate::parser::Parser;


pub fn function(body: String) -> String {
    format!(".add_fun(Fun::new(){})", Parser::from_cable_script(body).compile())
}

pub fn string(body: String) -> String {
    format!(".add_str(\"{}\")", body)
}

pub fn number(body: String) -> String {
    format!(".add_num(\"{}\")", body)
}

pub fn store_in_var(name: String) -> String {
    format!(".store(\"{}\")", name.trim())
}

pub fn load_from_var(name: String) -> String {
    format!(".load(\"{}\")", name.trim())
}

pub fn call_function(name: String) -> String {
    let mut result = "".to_string();
    result += &format!("{}.call_from_stack()", load_from_var(name));
    result
}

pub fn call_from_stack() -> String {
    let mut result = ".call_from_stack()".to_string();
    result
}

pub fn load_foreign_function(name: String) -> String {
    format!(".add_fun(Fun::new().call_foreign_function({}))", name)
}

