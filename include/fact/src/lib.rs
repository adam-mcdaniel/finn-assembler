use hlvm::function::*;

fn factorial_recursive(n: Value) -> Value {
    if n <= num("1") {
        return num("1");
    } else {
        return n.clone() * factorial_recursive(n-num("1"));
    }
}

pub fn factorial(v: Value) -> Value {
    let n = num(&factorial_recursive(v).as_number().to_string());
    n
}