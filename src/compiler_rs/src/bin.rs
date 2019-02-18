use cable_lang::object::*;



macro_rules! inline_foreign_function {
    ($input:ident, $body:block) => (
        Value::from_foreign_function(
            |$input| $body
        )
    )
}

macro_rules! wrap_foreign_function {
    ($function:expr) => (
        Value::from_foreign_function(
            |input: Value| {$function(input)}
        )
    )
}


macro_rules! __obj {
    ($object:ident, $name:meta~$value:expr) => (
        $object.set_attr(stringify!($name).to_string(), $value)
    );

    ($object:ident, $name:meta~$value:expr, $($pass_name:meta~$pass_value:expr),+) => (
        $object.set_attr(stringify!($name).to_string(), $value);
        __obj!($object, $($pass_name~$pass_value),+)
    )
}

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

macro_rules! string {
    ($value:expr) => (
        Value::from_str($value)
    )
}


macro_rules! float64 {
    ($value:expr) => (
        Value::from_f64($value)
    )
}

macro_rules! list {
    ($value:expr) => (
        Value::from_vector(
            $value.to_vec()
        )
    )
}


fn main() {
    // assert_eq!(
    //     Value::from_str("hello world!").as_string(),
    //     "hello world!".to_string(),
    // );
    
    // assert_eq!(
    //     Value::from_i32(1).as_number(),
    //     to_decimal(1),
    // );
    
    // inline_foreign_function!(x, {
    //     Value::from_number(
    //         x.as_number() * x.as_number()
    //         )
    // }).call_foreign_function(Value::from_i32(5)).println();


    // fn cube(x: Value) -> Value {
    //     println!("Received value: {}", x);
    //     x.clone() * x.clone() * x.clone()
    // }

    // wrap_foreign_function!(cube).call_foreign_function(Value::from_i32(3)).println();

    // list!([
    //     float64!(3.0),
    //     float64!(4.0),
    //     float64!(5.0)
    //     ]).println();
    // // list([string!(5)])
    // string!("hello world!").println();
    // float64!(4.5).println();
    let mut object = obj!(
        num~float64!("5.0".parse().unwrap()),
        object~obj!(
            object~obj!(
                string~string!("hello world!")
            )
        )
    );
    object.println();
    // object.get_attr("square".to_string()).call_foreign_function(float64!(5.0)).println();


    object
        .get_attr("object".to_string())
        .get_attr("object".to_string())

        .set_attr("string".to_string(), string!("goodbye world!"));
    
}