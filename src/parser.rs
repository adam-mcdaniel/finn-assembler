use crate::strings::*;
use crate::tokenizer::*;
use crate::tokenizer::Token::*;



pub fn number(s: &str) -> String {
    format!(".add_num(\"{}\")", s).clone()
}

pub fn string(s: &str) -> String {
    format!(".add_str({})", s).clone()
}

pub fn store(s: &str) -> String {
    format!(".store(\"{}\")", s).clone()
}

pub fn load(s: &str) -> String {
    format!(".load(\"{}\")", s).clone()
}

pub fn conditional() -> String {
    ".if_function()".to_string()
}

pub fn while_loop() -> String {
    ".while_function()".to_string()
}

pub fn begin_args() -> String {
    ")".to_string()
}

pub fn end_args() -> String {
    "".to_string()
}

pub fn begin_function() -> String {
    ".add_fun(Fun::new()".to_string()
}

pub fn end_function() -> String {
    ")".to_string()
}

pub fn call() -> String {
    ".call_from_stack()".to_string()
}

pub fn load_foreign_function(name: &str) -> String {
    format!(".add_foreign_fun({})", name)
}




pub fn add() -> String {".add()".to_string()}
pub fn sub() -> String {".sub()".to_string()}
pub fn mul() -> String {".mul()".to_string()}
pub fn div() -> String {".div()".to_string()}
pub fn equal() -> String {".eq()".to_string()}
pub fn not_equal() -> String {".eq().not()".to_string()}
pub fn less() -> String {".less()".to_string()}
pub fn greater() -> String {".greater()".to_string()}

            // "+" => Add,
            // "-" => Sub,
            // "*" => Mul,
            // "/" => Div,
            // "~" => Equal,
            // "^" => NotEqual,
            // "<" => Less,
            // ">" => Greater,


pub fn compile_tokens(tokens: Vec<Token>) -> String {
    // println!("{:?}", tokens);
    let mut result = "".to_string();
    
    let mut should_continue = false;
    for i in (0..tokens.len()) {

        if should_continue {
            should_continue = false;
            continue;
        }

        match &tokens[i] {
            Num(n) => {
                result += number(n).as_ref()
            },
            Str(s) => {
                result += string(s).as_ref()
            },
            Identifier(s) => {
                match tokens[(i+1) as usize] {
                    Assign => {
                        result += store(s).as_ref();
                        should_continue = true;
                    },
                    ForeignLoad => {
                        result += load_foreign_function(s).as_ref();
                        should_continue = true;
                    },
                    _ => {
                        result += load(s).as_ref();
                    }
                }
            },
            While => {
                result += &while_loop();
            },
            Conditional => {
                result += &conditional();
            },
            Call => {
                result += &call();
            },
            FunctionBegin => {
                result += &begin_function();
            },
            FunctionEnd => {
                result += &end_function();
            },

            // "+" => Add,
            // "-" => Sub,
            // "*" => Mul,
            // "/" => Div,
            // "~" => Equal,
            // "^" => NotEqual,
            // "<" => Less,
            // ">" => Greater,
            Add => {result += &add();},
            Sub => {result += &sub();},
            Mul => {result += &mul();},
            Div => {result += &div();},
            Equal => {result += &equal();},
            NotEqual => {result += &not_equal();},
            Less => {result += &less();},
            Greater => {result += &greater();},

            _ => {}
        };
    }

    result
}