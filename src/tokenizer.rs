use Token::*;
use crate::strings::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Str(String),
    Num(String),
    Identifier(String),
    // Function(Vec<Token>),
    // Call(Vec<Token>),
    // Args(Vec<Token>),
    Assign,
    ForeignLoad,
    EndStatement,
    NONE,


    FunctionBegin,
    FunctionEnd,
    Call,
    Conditional,
    While,

    Add, Div, Mul, Sub,
    Equal, NotEqual, Greater, Less, Not
    // CallBegin,
    // CallEnd,
    // ArgsBegin,
    // ArgsEnd,
}


pub fn tokenize(script: &str) -> Vec<Token> {
    // println!(
    //     "is num: {}", is_number("-9.0")
    // );

    // println!(
    //     "is ident: {}", is_identifier("a90")
    // );

    // println!(
    //     "is string: {}", is_string("\"\"")
    // );

    // println!(
    //     "{:?}",
    //     trim(split("hey {jude}, dont! make it bad", vec!["{", "}", "!", ","]))
    // );
        // .add_fun(Fun::new().add()).store("add")
        // .add_fun(Fun::new().sub()).store("sub")
        // .add_fun(Fun::new().mul()).store("mul")
        // .add_fun(Fun::new().div()).store("div")
        // .add_fun(Fun::new().less()).store("less")
        // .add_fun(Fun::new().greater()).store("greater")
        // .add_fun(Fun::new().eq()).store("eq")
        // .add_fun(Fun::new().eq().not()).store("noteq")
        // .add_fun(Fun::new().not()).store("not")
    let mut tokens = vec![];
    for token in trim(split(script, vec!["=", " ", "@", "{", "}", ",", "!", "&", "?", ";", "-", "+", "*", "/", "~", "<", ">", "^"])) {
        let result = match token.as_ref() {
            "{" => FunctionBegin,
            "}" => FunctionEnd,
            "=" => Assign,
            "+" => Add,
            "-" => Sub,
            "*" => Mul,
            "/" => Div,
            "~" => Equal,
            "^" => NotEqual,
            "<" => Less,
            ">" => Greater,
            "!" => Call,
            "&" => While,
            "?" => Conditional,
            "@" => ForeignLoad,
            ";" => EndStatement,
            "," => NONE,
            s => {
                if is_number(&s) {
                    Num(s.to_string())
                } else if is_string(&s) {
                    Str(s.to_string())
                } else if is_identifier(&s) {
                    Identifier(s.to_string())
                } else {
                    NONE
                }
            }
        };

        if result != NONE {
            tokens.push(result);
        }
        // tokens.push(result);
    }
    tokens
}