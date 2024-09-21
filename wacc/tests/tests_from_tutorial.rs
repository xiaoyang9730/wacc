use std::process::Command;
use OutputType::*;

enum OutputType {
    Stdout,
    Stderr,
}

fn test_compiler(args: &[&str], get_output: OutputType) -> String {
    let output = Command::new("../wacct/test_compiler").args(args).output().expect("That `test_compiler` command should be executed");
    assert!(output.status.success());
    match get_output {
        Stdout => String::from_utf8(output.stdout).expect("That `test_compiler` should output UTF-8"),
        Stderr => String::from_utf8(output.stderr).expect("That `test_compiler` should output UTF-8"),
    }
}

#[test]
fn check_setup() {
    println!("--- CH 0: CHECK SETUP ---");
    let output = test_compiler(&["--check-setup"], Stdout);
    println!("{output}");
    assert_eq!(output, "All system requirements met!\n");
}

#[test]
fn chapter_1() {
    println!("--- LEX ---");
    let output = test_compiler(&["target/debug/wacc", "--chapter", "1", "--stage", "lex"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));

    println!("--- PARSE ---");
    let output = test_compiler(&["target/debug/wacc", "--chapter", "1", "--stage", "parse"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));

    println!("--- CODEGEN ---");
    let output = test_compiler(&["target/debug/wacc", "--chapter", "1", "--stage", "codegen"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
}
