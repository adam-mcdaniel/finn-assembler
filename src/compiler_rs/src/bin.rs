#[allow(dead_code, unused_imports)]
use cable_lang::object::*;
use cable_lang::object::Instruction::*;

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

    // wrap_foreign_function!(cube).call_foreign_function(Value::from_i32(3)).println();

    // list!([
    //     float64!(3.0),
    //     float64!(4.0),
    //     float64!(5.0)
    //     ]).println();
    // // list([string!(5)])
    // string!("hello world!").println();
    // float64!(4.5).println();
    // let mut object = obj!(
    //     num~float64!("5.0".parse().unwrap()),
    //     object~obj!(
    //         object~obj!(
    //             string~string!("hello world!")
    //         )
    //     )
    // );
    // object.println();
    // // object.get_attr("square".to_string()).call_foreign_function(float64!(5.0)).println();


    // object
    //     .get_attr("object".to_string())
    //     .get_attr("object".to_string())

    //     .set_attr("string".to_string(), string!("goodbye world!"));

    StackFrame::from_instructions(
        list!([
            float64!(5.0),
            string!("test"),
            ins!(Add),
            ins!(Println),

            // float64!(4.0),
            num!("4.0"),
            string!("a"),
            ins!(Store),
            string!("a"),
            ins!(Load),
            ins!(Println),
            // float64!(5.0),
            num!("5.0"),
            string!("a"),
            ins!(Store),
            string!("a"),
            ins!(Load),
            ins!(Println),
            
            // string!(">>"),
            // wrap_foreign_function!(input),
            // ins!(Execute),
            // string!("you said: \""),
            // ins!(Print),
            // ins!(Print),
            // string!("\""),
            // ins!(Println),
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
            num!("1.0"),
            wrap_foreign_function!(sleep),
            ins!(Execute),
        ])
    ).run();
}