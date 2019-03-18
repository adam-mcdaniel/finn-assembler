# Finn-Assembler
The assembler for the Finn IR implemented using `HLVM v0.9.0`

# Finn IR

Finn IR is a minimal language that is meant to be targetted by other languages.

It compiles to Rust, which is then compiled to a binary.


## Features
* **Dynamic Typing** - Finn IR allows you to use dynamic typing without having to sacrifice too much speed.
* **128 bit Decimal Numbers** - We use 128 bit decimal numbers to get very large and precise values while maintaining speed during mathematical operations.
* **Foreign Function Interface** - Finn IR can access objects directly from included crates using the `@` operator after an identifier.
* **Object Oriented Programming** - The `:`, `>`, and `<` operators allow for simple object oriented programming.
* **Functional Programming** - Finn IR can use the lambda calculus, but not efficiently. Because it is not garbage collected, it does not have an infinite recursion depth either. However, this is great for closures, and other things.

## Examples

### Factorial

Calculates _n_ factorial.

`./fn examples/factorial.fn`

```rust
{ 
    n=
    1 total=
    {
        total n* total=

        1 n- n=
    } {1 n greater!} &

    total
} factorial=


1500 factorial! println!
```


### User Input

Gets user input from the command line, and prints out the result.

`./fn examples/input.fn --crates include/input`

```rust
">> " input@! user_input=

"You said: \"" print! user_input print! "\"" println!
```


### FFI 'macro'

Automates a lot of the syntax required to define a constructor function. It also demonstrates how you can use a Rust FFI to simulate 'macros' (they're not really macros).

`./fn examples/struct.fn --crates include/structs`

```rust
[
    0 "x"
    
    {
        self=

        self x: > 1 + self x: < self =
        "im a method!" println!
        self
    }    "method"

    {
        "im a static method!" println!
    }    "static_method"
] strukt@! s=


s x: > println!
s s method: > ! s=
s x: > println!
s static_method: > !
```


### Fibonacci

Calculates the fibonacci sequence.
`./fn examples/fibonacci.fn`

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

        "iteration " print! n print! "  " print! c println!
        1 n-n=
    } {0 n greater!} &

    c
} fib=


29404 fib!
```


### Object Oriented Programming

Demonstrates Object Oriented Programming in Finn IR.
`./fn examples/object.fn`

```rust
{
    $ self=

    x= y=
    x self x: < self=
    y self y: < self=

    {
        self= dx= dy=

        self x: > dx + self x: < self=
        self y: > dy + self y: < self=

        self
    } self move: < self=

    {
        self=

        "self:x " print! self x: > println!
        "self:y " print! self y: > println!

        self
    } self println: < self=

    self

} Sprite=


0 20.5 Sprite ! s=
s s println: > !


1 5 0- s s move: > ! s=
s s println: > !
```

