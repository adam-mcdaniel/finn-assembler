mod strings;
use strings::*;

mod tokenizer;
use tokenizer::*;

mod parser;
use parser::*;

use std::env;

fn main() {

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


    // println!(
    //     "Fun::new(){}.run()",
    //     Parser::from_cable_script(
    //         // "{ \"testing\" } name [\"hey jude\", 1] ".to_string()
    //         // "println[add[1, 1]]  println[mul[add[1, 1], 5]]".to_string()
    //         "cube! ".to_string()
    //     ).compile()
    // );

    // println!("{:?}", tokenize("a; println !;"));


    println!(
        "#[allow(unused_imports)]\nuse hlvm::function::{{Fun, Value, Object, ForeignFunction, string}};
        

use std::io;
use std::io::Write;
fn get_input(prompt: &str) -> String {{
    print!(\"{{}}\",prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {{
        Ok(_goes_into_input_above) => {{}},
        Err(_no_updates_is_fine) => {{}},
    }}
    input.trim().to_string()
}}

fn main() {{
    
    let input: ForeignFunction = |v| {{
        string(&get_input(&v.as_string()))
    }};


    Fun::new()
        .add_fun(Fun::new().print()).store(\"print\")
        .add_fun(Fun::new().println()).store(\"println\")
        .add_fun(Fun::new().add()).store(\"add\")
        .add_fun(Fun::new().sub()).store(\"sub\")
        .add_fun(Fun::new().mul()).store(\"mul\")
        .add_fun(Fun::new().div()).store(\"div\")
        .add_fun(Fun::new().less()).store(\"less\")
        .add_fun(Fun::new().greater()).store(\"greater\")
        .add_fun(Fun::new().eq()).store(\"eq\")
        .add_fun(Fun::new().eq().not()).store(\"noteq\")
        .add_fun(Fun::new().not()).store(\"not\")
    
    // START USER PROGRAM
    {}.run()
    
    // END USER PROGRAM
}}",
        compile_tokens(
            // tokenize("(a, b, c){mul[a, mul[b, c]]}   println[add[11.0, 2.0]]")
            // tokenize("(a){(){println[a]}}[add[2, 2]]")
            tokenize(&(env::args().collect::<Vec<_>>()[1]).to_string())
            )
    );
}