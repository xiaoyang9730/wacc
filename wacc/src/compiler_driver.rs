use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process::{Command, exit};

use crate::lexer::Lexer;

#[derive(Debug, Default)]
enum CompilerDriverOption {
    Lex = 0,
    Parse = 1,
    Codegen = 2,
    EmitAssembly = 3,
    #[default]
    All = 4,
}

#[derive(Debug, Default)]
pub struct CompilerDriver {
    option: CompilerDriverOption,
    filename: String,
}

impl CompilerDriver {
    pub fn config() -> Self {
        let mut cd = Self::default();
        for arg in env::args().skip(1) {
            match arg.as_str() {
                "--lex" => cd.option = CompilerDriverOption::Lex,
                "--parse" => cd.option = CompilerDriverOption::Parse,
                "--codegen" => cd.option = CompilerDriverOption::Codegen,
                "-S" => cd.option = CompilerDriverOption::EmitAssembly,
                option => if option.starts_with('-') {
                    eprintln!("[config] unsupported option `{option}`");
                    exit(1);
                } else {
                    cd.filename = String::from(option);
                },
            }
        }
        if cd.filename.len() == 0 {
            eprintln!("[config] no input files");
            exit(1);
        } else if !cd.filename.ends_with(".c") {
            eprintln!("[config] `{}` is not a .c file", cd.filename);
            exit(1);
        }
        println!("[config] {cd:#?}");
        cd
    }

    fn filename_preprocessed(&self) -> String {
        format!("{}.i", &self.filename[..self.filename.len()-2])
    }

    fn filename_assembly(&self) -> String {
        format!("{}.s", &self.filename[..self.filename.len()-2])
    }

    fn filename_output(&self) -> String {
        format!("{}", &self.filename[..self.filename.len()-2])
    }

    fn preprocess(&self) -> io::Result<()> {
        let options = ["-E", "-P", &self.filename, "-o", &self.filename_preprocessed()];
        gcc(&options)?;
        Ok(())
    }

    #[allow(unused)]
    fn assemble_and_link(&self) -> io::Result<()> {
        let options = [&self.filename_assembly(), "-o", &self.filename_output()];
        gcc(&options)?;
        Ok(())
    }

    pub fn run(&self) -> io::Result<()> {
        self.preprocess()?;

        let mut code = String::new();
        File::open(self.filename_preprocessed())?.read_to_string(&mut code)?;
        let mut lexer = Lexer::new(&code);
        while let Some(token) = lexer.next_token() {
            println!("[token] {token}");
        }

        // self.assemble_and_link()?;
        Ok(())
    }
}

fn gcc(options: &[&str]) -> io::Result<()> {
    print!("[command] gcc");
    options.iter().for_each(|op| print!(" {op}"));
    println!("");

    let output = Command::new("gcc").args(options).output()?;
    if output.stdout.len() > 0 {
        println!("--- stdout ---\n{}", String::from_utf8(output.stdout).expect("That output should be UTF-8"));
    }
    if output.stderr.len() > 0 {
        println!("--- stderr ---\n{}", String::from_utf8(output.stderr).expect("That output should be UTF-8"));
    }
    Ok(())
}
