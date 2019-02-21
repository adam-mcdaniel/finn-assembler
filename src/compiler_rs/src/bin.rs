#[allow(dead_code, unused_imports)]
use hlvm::object::*;
use hlvm::object::Instruction::*;
use hlvm::value::*;

#[allow(dead_code, unused_imports)]
use hlvm::runtime::*;


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
            num!("1"),
            num!("2"),
            num!("27"),
            list!([
                string!("a"),
                ins!(Store),
                list!([
                    string!("b"),
                    ins!(Store),
                    list!([
                        string!("c"),
                        ins!(Store),
                        string!("c"),
                        ins!(Load),
                    ]),
                    ins!(Call)
                ]),
                ins!(Call)
            ]),
            ins!(Call),
            ins!(Println),
        ])
    ).run();
}