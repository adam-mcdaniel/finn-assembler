use std::cmp::{min, max};
use crate::error::*;
use crate::stdout::*;
use crate::tokenizer::*;
use crate::tokenizer::Token::*;


const SNIPPET_SIZE: usize = 10;

fn print_snippet(tokens: &Vec<Token>, i: usize) {
    let i = max(SNIPPET_SIZE/2, i);
    sub_error_info(
        &format!("{}", tokens_to_string(tokens.clone()[max(i-SNIPPET_SIZE/2, 0)..min(i+SNIPPET_SIZE/2, tokens.len()-1)].to_vec()))
    );
}

fn find_unmatched_functions(tokens: &Vec<Token>) {
    sub_info(&format!("Checking for unmatched function delimiters..."));

    let mut index_stack = vec![];
    let mut count = 0;

    for (i, token) in tokens.iter().enumerate() {
        let change = match token {
            FunctionBegin => {
                index_stack.push(i);
                1
            },
            FunctionEnd => {
                index_stack.pop();
                -1
            },
            _ => {0}
        };

        count += change;
        if count < 0 {
            sub_error(
                &format!("Unmatched '}}' at instruction #{}", i)
            );
            print_snippet(tokens, i);
            throw();
            return;
        }
    }

    if count > 0 {
        let i = index_stack[index_stack.len()-1];
        sub_error(
            &format!("Unmatched '{{' at instruction #{}", i)
            // &format!("Unmatched '{{' at token #{}", index_stack[index_stack.len()-1])
        );
        print_snippet(tokens, i);
        throw();
    }
}


fn find_unmatched_lists(tokens: &Vec<Token>) {
    sub_info(&format!("Checking for unmatched list delimiters..."));

    let mut index_stack = vec![];
    let mut count = 0;

    for (i, token) in tokens.iter().enumerate() {
        let change = match token {
            ListBegin => {
                index_stack.push(i);
                1
            },
            ListEnd => {
                index_stack.pop();
                -1
            },
            _ => {0}
        };

        count += change;
        if count < 0 {
            sub_error(
                &format!("Unmatched ']' at instruction #{}", i)
            );
            print_snippet(tokens, i);
            throw();
            return;
        }
    }

    
    if count > 0 {
        let i = index_stack[index_stack.len()-1];
        sub_error(
            &format!("Unmatched '[' at instruction #{}", i)
        );
        print_snippet(tokens, i);
        throw();
    }
}



pub fn error_check(tokens: Vec<Token>) -> Vec<Token> {
    info(&format!("Performing compile time error checks"));

    find_unmatched_functions(&tokens);
    find_unmatched_lists(&tokens);

    if has_thrown_error() {
        error(
            &format!("Detected errors during compile time checks")
        );
        stop();
    } else {
        success("No errors detected");
    }

    return tokens;
}

