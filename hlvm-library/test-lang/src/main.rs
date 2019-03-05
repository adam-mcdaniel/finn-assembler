mod lang;
use lang::*;


fn main() {
    interpret("\"Hello world!\" !").run();
}
