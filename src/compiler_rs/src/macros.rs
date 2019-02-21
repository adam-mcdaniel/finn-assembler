#[allow(unused_macros)]
macro_rules! inline_foreign_function {
    ($input:ident, $body:block) => (
        Value::from_foreign_function(
            |$input| $body
        )
    )
}

#[allow(unused_macros)]
macro_rules! wrap_foreign_function {
    ($function:expr) => (
        Value::from_foreign_function(
            |input: Value| {$function(input)}
        )
    )
}

#[allow(unused_macros)]
macro_rules! __obj {
    ($object:ident, $name:meta~$value:expr) => (
        $object.set_attr(stringify!($name).to_string(), $value)
    );

    ($object:ident, $name:meta~$value:expr, $($pass_name:meta~$pass_value:expr),+) => (
        $object.set_attr(stringify!($name).to_string(), $value);
        __obj!($object, $($pass_name~$pass_value),+)
    )
}

#[allow(unused_macros)]
macro_rules! obj {
    ($($name:meta~$value:expr),+) => (
        (|| {
            let mut value = Value::new(Type::Instance, NOTHING.to_vec());
            __obj!(
                value, $($name~$value),+
            );
            return value
        })()
    )
}

#[allow(unused_macros)]
macro_rules! string {
    ($value:expr) => (
        Value::from_str($value)
    )
}

#[allow(unused_macros)]
macro_rules! float64 {
    ($value:expr) => (
        Value::from_f64($value)
    )
}

#[allow(unused_macros)]
macro_rules! num {
    ($value:expr) => (
        Value::from_number(num_from_str($value))
    )
}

#[allow(unused_macros)]
macro_rules! list {
    ($value:expr) => (
        Value::from_vector(
            $value.to_vec()
        )
    )
}

#[allow(unused_macros)]
macro_rules! ins {
    ($value:expr) => (
        Value::from_instruction(
            $value
        )
    )
}