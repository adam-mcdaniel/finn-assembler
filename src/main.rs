extern crate hlvm;
use hlvm::function::{Fun, string, num};

fn main() {

    let mut main = Fun::new()
        .add_data(
            Fun::new()
                .store("m")
                .add_data(
                    Fun::new()
                        .load("m")
                        .mul()
                        .as_value()
                )
                .as_value()
        )
        .store("multiply")

        .add_data(num("2"))
        .load("multiply")
        .call_from_stack()
        .store("double")

        .add_data(num("3"))
        .load("multiply")
        .call_from_stack()
        .store("triple")

        .add_data(string("5 doubled is ")).print()
        .add_data(num("5")).load("double")
        .call_from_stack().println()

        .add_data(string("7 doubled is ")).print()
        .add_data(num("7")).load("double")
        .call_from_stack().println()

        .add_data(string("5 tripled is ")).print()
        .add_data(num("5")).load("triple")
        .call_from_stack().println()

        .add_data(string("7 tripled is ")).print()
        .add_data(num("7")).load("triple")
        .call_from_stack().println()
        ;

    main.run();
}