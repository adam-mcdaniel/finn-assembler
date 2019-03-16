use std::fs;
use std::path::Path;
use std::process::Command;

use fs_extra::dir::*;


use crate::error::*;
use crate::stdout::*;


const OUTPUT_CRATE: &str = "out";
const CARGO_TOML: &str = "[package]
name = \"output\"
version = \"0.1.0\"
authors = [\"adam-mcdaniel <adam.mcdaniel17@gmail.com>\"]
edition = \"2018\"


[profile.release]
opt-level = 3
debug = false
rpath = false
lto = false
debug-assertions = false
codegen-units = 16
panic = 'unwind'
incremental = false
overflow-checks = false

[dependencies]
hlvm = {path=\"../hlvm\"}
# hlvm = \"0.8.0\"

num-traits = \"*\"
";


pub fn path_to_str(name: &str) -> String {
    match Path::new(&name.clone().to_string()).file_name() {
        Some(p) => {
            match p.to_str() {
                Some(os_str) => os_str.to_string(),
                None => {
                    sub_error(
                        &format!("{} is an invalid path", name)
                    );
                    throw();
                    "".to_string()
                }
            }
        },
        None => {
            sub_error(
                &format!("{} is an invalid path", name)
            );
            throw();
            "".to_string()
        }
    }
}


pub fn read_file(file: &str) -> String {
    match fs::read_to_string(file) {
        Ok(k) => k,
        Err(_) => {
            sub_error(
                &format!("Unable to read file '{}'", &file)
            );
            throw();
            "".to_string()
        }
    }
}


fn copy_folder(folder_path: &str, new_folder_path: &str) {
    sub_info(
        &format!("Including crate '{}'...", path_to_str(&folder_path))
    );

    let mut options = CopyOptions::new();
    options.copy_inside = true;

    match remove(new_folder_path) {_ => {}};

    match copy(folder_path, new_folder_path, &options) {
        Ok(_) => {
            sub_info(&format!("Successfully included crate '{}'", path_to_str(&folder_path)));
        },
        Err(e) => {
            sub_error(
                &format!("Unable to copy folder '{}' to '{}' because '{}'", &folder_path, &new_folder_path, e)
            );
            throw();
        }
    };
}


fn make_output_crate() {
    if Path::new(OUTPUT_CRATE).exists() {
        sub_info(
            &format!("Output crate already exists")
        );
        
    } else {
        sub_info(
            &format!("Creating output crate...")
        );

        Command::new("cargo")
            .current_dir(".")
            .arg("new")
            .arg(OUTPUT_CRATE)
            .output()
            .expect("Could not create output crate");

        sub_info(
            &format!("Created output crate")
        );
    }
}


fn write_to_file(file: String, contents: String) {
    match fs::write(&file, &contents) {
        Ok(_) => {
            sub_info(
                &format!("Wrote '{}...' to file '{}'", &contents[0..9], file)
            );
        },
        Err(e) => {
            sub_error(&format!("Could not write to file '{}' because {}", file, e));
            throw();
        }
    }
}


pub fn write_deps(crate_names: Vec<String>) {
    info(&format!("Including dependant crates..."));
    make_output_crate();

    let mut dependencies = "".to_string();

    for name in crate_names {
        let dep_name = path_to_str(&name);
        dependencies += &format!("{} = {{path=\"{}\"}}\n", dep_name, dep_name);

        copy_folder(&name, &(OUTPUT_CRATE.to_string() + "/" + &dep_name));
    }

    write_to_file(
        OUTPUT_CRATE.to_string() + "/Cargo.toml",
        CARGO_TOML.to_string() + &dependencies,
    );

    if has_thrown_error() {
        error(
            &format!("Could not include dependant crates due to errors")
        );
        stop();
    } else {
        success("Successfully included dependant crates");
    }
}


pub fn write_compiled_script(script: String) {
    info(&format!("Writing compiled script to output crate"));
    write_to_file(OUTPUT_CRATE.to_string() + "/src/main.rs", script);

    if has_thrown_error() {
        error(
            &format!("Could not write compiled script due to errors")
        );
        stop();
    } else {
        success("Successfully wrote compiled script");
    }
}
    


pub fn compile_output_crate() {
    info(&format!("Compiling output crate"));
    match Command::new("cargo")
            .current_dir(OUTPUT_CRATE)
            .arg("build")
            .arg("--release")
            .output() {

        Ok(k) => {
            if k.status.success() {
                sub_info("Built output crate");
            } else {
                sub_error("Could not compile output crate; did you include all dependant crates?");
                throw();
            }
        },
        Err(_) => {
            sub_error("Failed start cargo");
            throw();
        }
    }
        
    if has_thrown_error() {
        error(
            &format!("Could not compile output crate due to errors")
        );
        stop();
    } else {
        success("Successfully compiled output crate");
    }
}
