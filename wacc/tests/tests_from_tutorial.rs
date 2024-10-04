use std::process::Command;
use OutputType::*;

enum OutputType {
    Stdout,
    Stderr,
}

const TEST_COMPILER: &str = "../wacct/test_compiler";
const WACC: &str = "target/debug/wacc";

fn test_compiler(args: &[&str], get_output: OutputType) -> (bool, String) {
    print!("{TEST_COMPILER}");
    for arg in args {
        print!(" {arg}");
    }
    println!("");

    let output = Command::new(TEST_COMPILER).args(args).output().expect("That `test_compiler` command should be executed");
    (
        output.status.success(),
        match get_output {
            Stdout => String::from_utf8(output.stdout).expect("That `test_compiler` should output UTF-8"),
            Stderr => String::from_utf8(output.stderr).expect("That `test_compiler` should output UTF-8"),
        },
    )
}

fn check_setup() {
    println!("=== CH 0: CHECK SETUP ===\n");

    let (success, output) = test_compiler(&["--check-setup"], Stdout);
    println!("{output}");
    assert_eq!(output, "All system requirements met!\n");
    assert!(success);
}

fn chapter_1() {
    println!("=== CH 1: A MINIMAL COMPILER ===\n");

    println!("[LEX]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "1", "--stage", "lex"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);

    println!("[PARSE]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "1", "--stage", "parse"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);

    println!("[CODEGEN]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "1", "--stage", "codegen"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);

    println!("[WHOLE COMPILER]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "1"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);
}

fn chapter_2() {
    println!("=== CH 2: Unary Operators ===\n");

    println!("[LEX]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "2", "--stage", "lex"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);

    println!("[PARSE]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "2", "--stage", "parse"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);

    println!("[TACKY]");
    let (success, output) = test_compiler(&[WACC, "--chapter", "2", "--stage", "tacky"], Stderr);
    println!("{output}");
    assert!(output.ends_with("OK\n"));
    assert!(success);
}

#[test]
fn all() {
    // check_setup();
    // chapter_1();
    chapter_2();
}
