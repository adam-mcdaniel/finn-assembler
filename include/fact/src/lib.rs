use hlvm::function::*;
use bigdecimal::ToPrimitive;

fn factorial_recursive(n: u128) -> u128 {
    match n {
        0 => 1,
        _ => n * factorial_recursive(n-1)
    }
}

pub fn factorial(v: Value) -> Value {
    num(
        &factorial_recursive(
            match v.as_number().to_u128() {
                Some(n) => n,
                None => 0 as u128
            }
        ).to_string()
    )
}