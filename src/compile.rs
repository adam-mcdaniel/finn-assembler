use crate::file::*;
use crate::parser::*;
use crate::tokenizer::*;


pub fn compile_script(crate_names: Vec<String>, script: String) -> String {
    let mut imports = "".to_string();

    for name in crate_names {
        imports += &("use ".to_string() + &path_to_str(&name) + "::*;\n");
    }

format!("#[allow(unused_imports)]\nuse hlvm::function::*;
{}
#[allow(unused_variables)]
fn main() {{
    Fun::new()
        .add_fun(Fun::new().print()).store(\"print\")
        .add_fun(Fun::new().println()).store(\"println\")
        .add_fun(Fun::new().add()).store(\"add\")
        .add_fun(Fun::new().sub()).store(\"sub\")
        .add_fun(Fun::new().mul()).store(\"mul\")
        .add_fun(Fun::new().div()).store(\"div\")
        .add_fun(Fun::new().less()).store(\"less\")
        .add_fun(Fun::new().greater()).store(\"greater\")
        .add_fun(Fun::new().eq()).store(\"eq\")
        .add_fun(Fun::new().eq().not()).store(\"noteq\")
        .add_fun(Fun::new().not()).store(\"not\")
        .add_fun(
            Fun::new()
                .get_parameter(\"index\")
                .get_parameter(\"list\")
                .load(\"index\")
                .load(\"list\")
                .index()
        ).store(\"at\")
        .add_fun(Fun::new().pop_list()).store(\"pop\")
        .add_fun(Fun::new().append_list()).store(\"push\")
    
        // START USER PROGRAM
        {}.run()

        // END USER PROGRAM
}}", imports, compile_tokens(tokenize(&script)))
}
        
 