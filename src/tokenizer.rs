use Token::*;
use crate::error::*;
use crate::stdout::*;
use crate::strings::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Str(String),
    Num(String),
    Identifier(String),
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
    Equal, NotEqual,
    // Greater, Less, Not,

    Instance,
    Dot,
    GetAttr,
    SetAttr,
}


pub fn tokenize(script: &str) -> Vec<Token> {
    info("Tokenizing input script");

    let mut tokens = vec![];
    for token in trim(split(script, vec!["=", " ", "@", "{", "}", "(", ")", "!", "&", "?", ";", "-", "+", "*", "/", "~", "<", ">", "^", "$", ":"])) {
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
            "<" => SetAttr,
            ">" => GetAttr,
            ":" => Dot,
            "!" => Call,
            "$" => Instance,
            "&" => While,
            "?" => Conditional,
            "@" => ForeignLoad,
            ";" => EndStatement,
            s => {
                if is_number(&s) {
                    Num(s.to_string())
                } else if is_string(&s) {
                    Str(s.to_string())
                } else if is_identifier(&s) {
                    if s.contains(" ") {
                        sub_error(
                            &format!("Invalid identifier '{}'", s)
                            );
                        throw();
                    }
                    Identifier(s.to_string())
                } else {
                    NONE
                }
            }
        };

        if result != NONE {
            tokens.push(result);
        }
    }

    if !tokens.iter().any(|v| v.clone() == Call) {
        sub_warn("No function calls found in tokenized script")
    };

        
    if has_thrown_error() {
        error(
            &format!("Could not tokenize script due to errors")
        );
        stop();
    } else {
        success("Successfully tokenized script");
    }
    tokens
}