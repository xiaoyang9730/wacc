mod compiler_driver;
use compiler_driver::CompilerDriver;

mod lexer;
mod parser;
mod ast_nodes;
mod codegen;
mod emit;

use std::env;

fn main() {
    let compiler_driver = CompilerDriver::config(env::args());
    compiler_driver.run();
}
