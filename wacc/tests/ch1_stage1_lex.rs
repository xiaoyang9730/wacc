use std::process::Command;

#[test]
fn ch1_stage1_lex() {
    let output = Command::new("../wacct/test_compiler").args(["target/debug/wacc", "--chapter", "1", "--stage", "lex"]).output().unwrap();
    let output = String::from_utf8(output.stderr).unwrap();
    println!("{output}");
    assert!(output.ends_with("OK\n"));
}
