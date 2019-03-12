# cable-lang
A dynamically typed, multi-paradigm programming language


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