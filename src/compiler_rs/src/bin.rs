#[allow(dead_code, unused_imports)]
use cable_lang::object::*;
use cable_lang::object::Instruction::*;
use cable_lang::value::*;

#[allow(dead_code, unused_imports)]
use cable_lang::runtime::*;


use std::io;
use std::io::Write;
use std::{thread, time};
use bigdecimal::ToPrimitive;
include!("macros.rs");


fn main() {
    fn cube(x: Value) -> Value {
        println!("Received value: {}", x);
        x.clone() * x.clone() * x.clone()
    }

    fn half(x: Value) -> Value {
        println!("Received value: {}", x);
        x / num!("2")
    }

    fn input(x: Value) -> Value {
        print!("{}", x);
        io::stdout().flush().unwrap();
        let mut input_string = String::new();
        match io::stdin().read_line(&mut input_string) {
            Ok(_) => {
                input_string.pop();
                string!(&input_string)
            }
            Err(_) => {
                string!("")
            }
        }
    }

    fn sleep(time: Value) -> Value {
        println!("Sleeping for {} seconds", time);
        let millis = time::Duration::from_millis(1000 * time.as_number().to_f64().unwrap() as u64);

        thread::sleep(millis);
        println!("Done!");
        return Value::from_nothing();
    }


    StackFrame::from_instructions(
        list!([
            // float64!(5.0),
            // string!("test"),
            // ins!(Add),
            // ins!(Println),

            num!("4.0"),
            string!("a"),
            ins!(Store),
            string!("a"),
            ins!(Load),
            ins!(Println),
            num!("5.0"),
            string!("a"),
            ins!(Store),
            string!("a"),
            ins!(Load),
            ins!(Println),
            
            string!("Hello"),
            inline_foreign_function!(x, {x + string!(" world!")}),
            ins!(Execute),
            ins!(Println),

            num!("345678907654.34567898765432"),
            wrap_foreign_function!(cube),
            ins!(Execute),
            string!("the cube is: \""),
            ins!(Print),
            ins!(Print),
            string!("\""),
            ins!(Println),
            num!("345678907654.34567898765432"),
            wrap_foreign_function!(half),
            ins!(Execute),
            string!("the half is: \""),
            ins!(Print),
            ins!(Print),
            string!("\""),
            ins!(Println),
        ])
    ).run();
}