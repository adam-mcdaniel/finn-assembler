use std::process::exit;


static mut ERROR: bool = false;

pub fn throw() {
    unsafe {
        ERROR = true;
    }
}

pub fn has_thrown_error() -> bool {
    unsafe {
        return ERROR;
    }
}

pub fn stop() {
    exit(1);
}