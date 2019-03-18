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
