use hlvm::function::*;
use hlvm_runtime::stack::StackFrame;


pub fn map_rs(fun_and_list: Value) -> Value {
    let vec_fun_and_list = fun_and_list.as_list();
    let fun = &vec_fun_and_list[0];
    let l = &vec_fun_and_list[1];

    let mut result_vec: Vec<Value> = vec![];

    for item in l.as_list() {

        let mut vm = StackFrame::from_instructions(
            Fun::new()
            // std
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
            .add_fun(
                Fun::new()
                    .get_parameter("index")
                    .get_parameter("list")
                    .load("index")
                    .load("list")
                    .index()
            ).store("at")
            .add_fun(Fun::new().pop_list()).store("pop")
            .add_fun(Fun::new().append_list()).store("push")
            // end std
            .add_data(item)
            .add_data(fun.clone())
            .call_from_stack()
            .as_value()
        );

        vm.run();

        result_vec.push(vm.return_value());
        // let mut stack_vec;
        // let stack_vec = temp_fun;
    }

    // Fun::new().add_fun(
    //     Fun::new()
    // ).run();

    let result = list(&result_vec.clone());


    result
}



pub fn range_rs(n: Value) -> Value {
    let mut result_vec = vec![];


    for i in 0..n.as_number().to_usize() {
        result_vec.push(num(&i.to_string()));
    }

    let result = list(&result_vec.clone());

    result
}
