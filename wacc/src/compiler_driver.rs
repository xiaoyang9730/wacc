use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

use crate::lexer::{Lexer, Tokens};
use crate::parser::Parser;
use crate::ast_nodes::{AsmProgram, CProgram, TackyProgram};
use crate::tackygen::gen_tacky_program;
use crate::codegen::gen_asm_program;
use crate::emit::emit_asm_program;

use CompilerDriverOption::*;

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub enum CompilerDriverOption {
    EmitReferenceAssembly = 0,
    Lex = 1,
    Parse = 2,
    Tacky = 3,
    Codegen = 4,
    EmitAssembly = 5,
    #[default]
    All = 6,
}

#[derive(Default)]
pub struct CompilerDriver {
    option: CompilerDriverOption,
    filename: String,
}

impl CompilerDriver {
    pub fn set_option(&mut self, option: CompilerDriverOption) {
        self.option = option;
    }

    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.into();
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

    fn check_config(&self) -> Result<(), String> {
        println!("Option: {:?}", self.option);
        println!("Filename: `{}`", self.filename);
        if self.filename.is_empty() {
            return Err("No input file".into());
        }
        if !self.filename.ends_with(".c") {
            return Err(format!("Filename `{}` should end with \".c\"", self.filename));
        }
        Ok(())
    }

    fn emit_reference_assembly(&self) -> Result<(), String> {
        println!("--- Stage: EMIT REFERENCE ASSEMBLY ---");
        gcc(&["-S", "-O", "-fno-asynchronous-unwind-tables", "-fcf-protection=none", &self.filename, "-o", &self.filename_assembly()])
    }

    fn preprocess(&self) -> Result<(), String> {
        println!("--- Stage: PREPROCESS ---");
        gcc(&["-E", "-P", &self.filename, "-o", &self.filename_preprocessed()])
    }

    fn lex(&self) -> Result<Lexer, String> {
        println!("--- Stage: LEX ---");

        let mut lexer = Lexer::default();
        File::open(self.filename_preprocessed())
            .map_err(|e| format!("Failed to open {}: {e}", self.filename_preprocessed()))?
            .read_to_string(lexer.get_src_mut())
            .map_err(|e| format!("Failed to read preprocessed file: {e}"))?;
        Command::new("rm").arg(&self.filename_preprocessed()).status()
            .map_err(|e| format!("Failed to delete `{}`: {e}", self.filename_preprocessed()))?;

        for token in lexer.tokens() {
            println!("token: {}", token?);
        }
        Ok(lexer)
    }

    fn parse(&self, tokens: Tokens) -> Result<CProgram, String> {
        println!("--- Stage: PARSE ---");
        let c_program = Parser::from(tokens).parse()?;
        println!("Abstract syntax tree:\n{c_program:#?}");
        Ok(c_program)
    }

    fn tacky(&self, c_program: CProgram) -> TackyProgram {
        println!("--- Stage: PARSE ---");
        let tacky = gen_tacky_program(c_program);
        println!("Tacky:\n{tacky:#?}");
        tacky
    }

    fn codegen(&self, tacky_program: TackyProgram) -> AsmProgram {
        println!("--- Stage: CODEGEN ---");
        let asm_program = gen_asm_program(tacky_program);
        println!("Generated assembly program:\n{asm_program:#?}");
        asm_program
    }

    fn emit_assembly(&self, asm_program: AsmProgram) -> Result<(), String> {
        println!("--- Stage: EMIT ASSEMBLY ---");
        let asm_code = emit_asm_program(asm_program);
        println!("Emit assembly code:\n{asm_code}");

        let mut asm_file = File::create(self.filename_assembly())
            .map_err(|e| format!("Failed to create file `{}`: {e}", self.filename_assembly()))?;
        writeln!(asm_file, "{asm_code}")
            .map_err(|e| format!("Failed to write assembly code to `{}`: {e}", self.filename_assembly()))
    }

    fn assemble_and_link(&self) -> Result<(), String> {
        println!("--- Stage: ASSEMBLE & LINK ---");
        gcc(&[&self.filename_assembly(), "-o", &self.filename_output()])?;
        Command::new("rm").arg(&self.filename_assembly()).status().map(|_| {})
            .map_err(|e| format!("Failed to delete `{}`: {e}", self.filename_assembly()))
    }

    pub fn run(&mut self) -> Result<(), String> {
        self.check_config()?;

        if self.option == EmitReferenceAssembly {
            self.emit_reference_assembly()
                .map_err(|e| format!("`Emit Referenct Assembly` stage failed: {e}"))?;
        }

        if self.option < Lex { return Ok(()) }
        self.preprocess()
            .map_err(|e| format!("`Preprocess` stage failed: {e}"))?;
        let lexer = self.lex()
            .map_err(|e| format!("`Lex` stage failed: {e}"))?;

        if self.option < Parse { return Ok(()) }
        let c_program = self.parse(lexer.tokens())
            .map_err(|e| format!("`Parse` stage failed: {e}"))?;

        if self.option < Tacky { return Ok(()) }
        let tacky_program = self.tacky(c_program);

        if self.option < Codegen { return Ok(()) }
        let asm_program = self.codegen(tacky_program);

        if self.option < EmitAssembly { return Ok(()) }
        self.emit_assembly(asm_program)
            .map_err(|e| format!("`Emit assembly` stage failed: {e}"))?;

        if self.option < All { return Ok(()) }
        self.assemble_and_link()
            .map_err(|e| format!("`Assemble and link` stage failed: {e}"))
    }
}

fn gcc(options: &[&str]) -> Result<(), String> {
    println!("gcc{}", options.iter().map(|op| format!(" {op}")).collect::<String>());

    let output = Command::new("gcc").args(options).output()
        .map_err(|e| format!("Failed to execute gcc process: {e}"))?;

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8(output.stdout).unwrap_or("GCC stdout isn't UTF-8".into()));
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8(output.stderr).unwrap_or("GCC stderr isn't UTF-8".into()));
    }
    if !output.status.success() {
        return Err(format!("GCC command failed{}", output.status.code().map(|c| format!(" with code: {c}")).unwrap_or_default()));
    }
    Ok(())
}
