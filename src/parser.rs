use crate::strings::*;
use crate::tokenizer::*;
use crate::tokenizer::Token::*;



pub fn number(s: &str) -> String {
    format!("\n\t\t.add_num(\"{}\")", s).clone()
}

pub fn string(s: &str) -> String {
    format!("\n\t\t.add_str({})", s).clone()
}

pub fn store(s: &str) -> String {
    format!("\n\t\t.store(\"{}\")", s).clone()
}

pub fn load(s: &str) -> String {
    format!("\n\t\t.load(\"{}\")", s).clone()
}

pub fn conditional() -> String {
    "\n\t\t.if_function()".to_string()
}

pub fn while_loop() -> String {
    "\n\t\t.while_function()".to_string()
}

pub fn begin_args() -> String {
    ")".to_string()
}

pub fn end_args() -> String {
    "".to_string()
}

pub fn begin_function() -> String {
    "\n\t\t.add_fun(Fun::new()".to_string()
}

pub fn end_function() -> String {
    ")".to_string()
}

pub fn call() -> String {
    "\n\t\t.call_from_stack()".to_string()
}

pub fn load_foreign_function(name: &str) -> String {
    format!("\n\t\t.add_foreign_fun({})", name)
}



pub fn get_attr() -> String {"\n\t\t.get_attr()".to_string()}
pub fn set_attr() -> String {"\n\t\t.set_attr()".to_string()}

pub fn instance() -> String {"\n\t\t.add_obj()".to_string()}
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
    let mut in_attrs = false;
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
                    Dot => {
                        result += string(&("\"".to_string() + s + "\"")).as_ref();
                        should_continue = true;
                    },
                    _ => {
                        // match tokens[(i-1) as usize] {
                        //     Dot => result += string(&("\"".to_string() + s + "\"")).as_ref(),
                        //     _ => result += load(s).as_ref()
                        // };
                        result += load(s).as_ref();
                    }
                }
            },
            GetAttr => {
                result += &get_attr();
            },
            SetAttr => {
                result += &set_attr();
            },
            Instance => {
                result += &instance();
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