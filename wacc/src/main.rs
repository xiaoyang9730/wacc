mod compiler_driver;
use compiler_driver::{CompilerDriver, CompilerDriverOption::*};

mod lexer;
mod parser;
mod ast_nodes;
mod tackygen;
mod codegen;
mod emit;

use std::env::args;
use std::process::exit;

fn main() {
    let mut compiler_driver = CompilerDriver::default();

    for arg in args().skip(1) {
        match arg.as_str() {
            "-Sref"     => compiler_driver.set_option(EmitReferenceAssembly),
            "--lex"     => compiler_driver.set_option(Lex),
            "--parse"   => compiler_driver.set_option(Parse),
            "--codegen" => compiler_driver.set_option(Codegen),
            "--tacky"   => compiler_driver.set_option(Tacky),
            "-S"        => compiler_driver.set_option(EmitAssembly),
            option => {
                if option.starts_with('-') {
                    eprintln!("Invalid option `{option}`");
                    exit(1);
                }
                compiler_driver.set_filename(option)
            }
        }
    }

    if let Err(e) = compiler_driver.run() {
        eprintln!("Failed: {e}");
        exit(1);
    }
    println!("Succeeded");
}
