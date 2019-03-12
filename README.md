# finn-assembler
The assembler for the Finn IR implemented using my HLVM in Rust

# Finn IR

Finn IR is a minimal language that is meant to be targetted by other languages.

It compiles to Rust, which is then compiled to a binary.

Finn IR can import Rust functions extremely easy, but I haven't added an include function / opcode yet, so you can't just include your crate. However, you can edit the source and test it for yourself.


## Foreign Function Interface

This is an example of the compiled Rust source that implements a foreign function.

```rust

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

fn main() {
    // a wrapper for our get_input rust function
    let input: ForeignFunction = |v| {
        string(&get_input(&v.as_string()))
    };


    Fun::new()
        // ...compiled user program here
        .run();
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