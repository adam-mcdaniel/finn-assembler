# Finn-Assembler
The assembler for the Finn IR implemented using my HLVM in Rust

# Finn IR

Finn IR is a minimal language that is meant to be targetted by other languages.

It compiles to Rust, which is then compiled to a binary.

Finn IR can import Rust functions extremely easily, but I haven't added an include function / opcode yet, so you can't just include your crate. However, you can edit the source and test it for yourself.


## Foreign Function Interface

This is an example of the compiled Rust source that implements a foreign function.

```rust
#[allow(unused_imports)]
use hlvm::function::*;
use include_test::*;

// These functions are found in the included `include_test` crate
// 
// use std::io;
// use std::io::Write;
// pub fn get_input(prompt: &str) -> String {
//     print!("{}",prompt);
//     io::stdout().flush().unwrap();
//     let mut input = String::new();
//     match io::stdin().read_line(&mut input) {
//         Ok(_goes_into_input_above) => {},
//         Err(_no_updates_is_fine) => {},
//     }
//     input.trim().to_string()
// }
// 
// 
// pub fn input(v: Value) -> Value {
//     string(&get_input(&v.as_string()))
// }

#[allow(unused_variables)]
fn main() {
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
        
		.add_str(">> ")
		.add_foreign_fun(input) // push foreign function `input` to stack
		.call_from_stack()
		.store("user_input")
		.add_str("You said: '")
		.load("print")
		.call_from_stack()
		.load("user_input")
		.load("print")
		.call_from_stack()
		.add_str("'")
		.load("println")
		.call_from_stack().run()

        // END USER PROGRAM
}
```


All you would have to do to use that foreign function is put the `@` symbol after an identifier to specify it as a foreign function.

```rust
">> " input@! user_input=

"You said: " print! user_input println!
```

`input@` refers to our foreign function, and `!` calls it with the argument `">> "`.


## Examples

### Fibonacci
```rust
{
    n=

    1 a=
    0 b=
    0 c=

    {
        a b+ c=
        b a=
        c b=

        c println!

        1 n-n=
    } {1 n >} &

    c
} fib=


1000 fib!
```

### Factorial
```rust
{ 
    n=
    1 total=
    {
        total n* total=

        1 n- n=
    } {1 n >} &

    total
} factorial=


10000 factorial! println!
```

### Lambda Calculus
```rust
{a= {b= a}} True= 
{a= {b= b}} False= 

{p= {q= p q p!!}} And=


{ "is false!" println! } { "is true!" println! } True True And!!!!!
```