mod compiler_driver;
use compiler_driver::CompilerDriver;

mod lexer;

fn main() {
    let cd = CompilerDriver::config();
    if let Err(e) = cd.run() {
        eprintln!("[main] Failed to run compiler driver: {e}");
    }
}
