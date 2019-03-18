use Token::*;
use crate::error::*;
use crate::stdout::*;
use crate::strings::*;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Str(String),
    Num(String),
    Identifier(String),
    UnExpected(String),
    NONE,
    Assign,
    ForeignLoad,
    EndStatement,

    ListBegin,
    ListEnd,
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


pub fn tokens_to_string(tokens: Vec<Token>) -> String {
    // info("Tokenizing input script");
    let mut result = "".to_string();
    for token in &tokens {
        result += &(match token {
            FunctionBegin => "{".to_string(),
            FunctionEnd => "}".to_string(),
            ListBegin => "[".to_string(),
            ListEnd => "]".to_string(),
            Assign => "=".to_string(),
            Add => "+".to_string(),
            Sub => "-".to_string(),
            Mul => "*".to_string(),
            Div => "/".to_string(),
            Equal => "~".to_string(),
            NotEqual => "^".to_string(),
            SetAttr => "<".to_string(),
            GetAttr => ">".to_string(),
            Dot => ":".to_string(),
            Call => "!".to_string(),
            Instance => "$".to_string(),
            While => "&".to_string(),
            Conditional => "?".to_string(),
            ForeignLoad => "@".to_string(),
            EndStatement => ";".to_string(),
            Num(s) => s.to_string(),
            Str(s) => s.to_string(),
            UnExpected(s) => s.to_string(),
            Identifier(s) => s.to_string(),
            NONE => "".to_string(),
        } + " ");
    }
    result
}


pub fn tokenize(script: &str) -> Vec<Token> {
    info("Tokenizing input script");

    let mut tokens = vec![];
    for token in trim(split(script, vec![",", "=", " ", "@", "{", "}", "[", "]", "(", ")", "!", "&", "?", ";", "-", "+", "*", "/", "~", "<", ">", "^", "$", ":"])) {
        let result = match token.as_ref() {
            "{" => FunctionBegin,
            "}" => FunctionEnd,
            "[" => ListBegin,
            "]" => ListEnd,
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
            "," => UnExpected(",".to_string()),
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
                } else if s == "" {
                    NONE
                } else {
                    UnExpected(s.to_string())
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