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
    .add_num("0").store("n").add_fun(Fun::new().add_num("1").load("n").add().store("n").add_fun(Fun::new().store("a").add_fun(Fun::new().store("b").load("a"))).store("True").add_fun(Fun::new().store("a").add_fun(Fun::new().store("b").load("b"))).store("False").add_fun(Fun::new().store("p").add_fun(Fun::new().store("q").load("p").load("q").load("p").call_from_stack().call_from_stack())).store("And").add_fun(Fun::new().add_str("is false!").load("println").call_from_stack()).add_fun(Fun::new().add_str("is true!").load("println").call_from_stack()).load("True").load("True").load("And").call_from_stack().call_from_stack().call_from_stack().call_from_stack().call_from_stack().add_str("n is ").load("print").call_from_stack().load("n").load("println").call_from_stack()).add_fun(Fun::new().add_num("1000").load("n").less()).while_function().run()
    
    // END USER PROGRAM
}
