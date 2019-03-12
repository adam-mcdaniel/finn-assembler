#[allow(unused_imports)]
use hlvm::function::{Fun, Value, Object, ForeignFunction, string};
        

use std::io;
use std::io::Write;
fn get_input(prompt: &str) -> String {
    print!("{}",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

#[allow(unused_variables)]
fn main() {
    
    let input: ForeignFunction = |v| {
        string(&get_input(&v.as_string()))
    };


    Fun::new()
        .add_fun(Fun::new().print()).store("print")
        .add_fun(Fun::new().println()).store("println")
        .add_fun(Fun::new().add()).store("add")
        .add_fun(Fun::new().sub()).store("sub")
        .add_fun(Fun::new().mul()).store("mul")
        .add_fun(Fun::new().div()).store("div")
        .add_fun(Fun::new().less()).store("less")
        .add_fun(Fun::new().greater()).store("greater")
        .add_fun(Fun::new().eq()).store("eq")
        .add_fun(Fun::new().eq().not()).store("noteq")
        .add_fun(Fun::new().not()).store("not")
    
    // START USER PROGRAM
    .add_fun(Fun::new().store("n").add_num("1").store("a").add_num("0").store("b").add_num("0").store("c").add_fun(Fun::new().load("a").load("b").add().store("c").load("b").store("a").load("c").store("b").load("c").load("println").call_from_stack().add_num("1").load("n").sub().store("n")).add_fun(Fun::new().add_num("1").load("n").greater()).while_function().load("c")).store("fib").add_num("1000").load("fib").call_from_stack().run()
    
    // END USER PROGRAM
}
