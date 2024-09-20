mod compiler_driver;
use compiler_driver::CompilerDriver;

mod lexer;
mod parser;

use std::env;

fn main() {
    let compiler_driver = CompilerDriver::config(env::args());
    compiler_driver.run();
}
