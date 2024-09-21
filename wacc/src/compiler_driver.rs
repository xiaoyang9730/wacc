use std::env::Args;
use std::fs::File;
use std::io::{self, Read};
use std::process::{Command, exit};

use crate::lexer::{Lexer, Tokens};
use crate::parser::Parser;

#[derive(Debug, Default, PartialEq, PartialOrd)]
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
    pub fn config(args: Args) -> Self {
        let mut cd = Self::default();
        for arg in args.skip(1) {
            match arg.as_str() {
                "--lex" => cd.option = CompilerDriverOption::Lex,
                "--parse" => cd.option = CompilerDriverOption::Parse,
                "--codegen" => cd.option = CompilerDriverOption::Codegen,
                "-S" => cd.option = CompilerDriverOption::EmitAssembly,
                option => if option.starts_with('-') {
                    eprintln!("[compiler driver] unsupported option `{option}`");
                    exit(1);
                } else {
                    cd.filename = String::from(option);
                },
            }
        }
        if cd.filename.len() == 0 {
            eprintln!("[compiler driver] no input files");
            exit(1);
        } else if !cd.filename.ends_with(".c") {
            eprintln!("[compiler driver] `{}` is not a .c file", cd.filename);
            exit(1);
        }
        println!("[compiler driver] {cd:#?}");
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
        println!("[compiler driver] --- Stage: PREPROCESS ---");
        gcc(&["-E", "-P", &self.filename, "-o", &self.filename_preprocessed()])
    }

    fn lex(&self) -> io::Result<Lexer> {
        println!("[compiler driver] --- Stage: LEX ---");

        let mut lexer = Lexer::default();
        File::open(self.filename_preprocessed())?
            .read_to_string(lexer.get_src_mut())?;
        for token in lexer.tokens() {
            println!("[compiler driver] token: {token}");
        }
        Ok(lexer)
    }

    fn parse(&self, tokens: Tokens) {
        println!("[compiler driver] --- Stage: PARSE ---");

        let program = Parser::from(tokens).parse();
        println!("[compiler driver] Abstract syntax tree:\n{program:#?}");
        // unimplemented!("--parse option")
    }

    fn codegen(&self) {
        println!("[compiler driver] --- Stage: CODEGEN ---");
        todo!("--codegen option");
    }

    fn emit_assembly(&self) {
        println!("[compiler driver] --- Stage: EMIT ASSEMBLY ---");
        todo!("-S option");
    }

    fn assemble_and_link(&self) -> io::Result<()> {
        println!("[compiler driver] --- Stage: ASSEMBLE & LINK ---");
        gcc(&[&self.filename_assembly(), "-o", &self.filename_output()])
    }

    pub fn run(&self) {
        if let Err(e) = self.preprocess() {
            eprintln!("[compiler driver] Preprocess failed: {e}");
            exit(1);
        }

        let lexer = match self.lex() {
            Ok(lexer) => lexer,
            Err(e) => {
                eprintln!("[compiler driver] Lex failed: {e}");
                exit(1);
            }
        };

        if self.option >= CompilerDriverOption::Parse {
            self.parse(lexer.tokens());
        }
        if self.option >= CompilerDriverOption::Codegen {
            self.codegen();
        }
        if self.option >= CompilerDriverOption::EmitAssembly {
            self.emit_assembly();
        }
        if self.option >= CompilerDriverOption::All {
            if let Err(e) = self.assemble_and_link() {
                eprintln!("[compiler driver] Assemble and link failed: {e}");
                exit(1);
            }
        }
    }
}

fn gcc(options: &[&str]) -> io::Result<()> {
    print!("[compiler driver] gcc");
    options.iter().for_each(|op| print!(" {op}"));
    println!("");

    let output = Command::new("gcc").args(options).output()?;
    if output.stdout.len() > 0 {
        println!("--- stdout ---\n{}", String::from_utf8(output.stdout).expect("That GCC stdout should be UTF-8"));
    }
    if output.stderr.len() > 0 {
        eprintln!("--- stderr ---\n{}", String::from_utf8(output.stderr).expect("That GCC stderr should be UTF-8"));
    }
    if !output.status.success() {
        eprintln!("[compiler driver] GCC command failed with exit code: {:?}", output.status.code());
        exit(1);
    }

    Ok(())
}
