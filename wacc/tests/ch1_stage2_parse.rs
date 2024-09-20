use std::process::Command;

#[test]
fn ch1_stage2_parse() {
    let output = Command::new("../wacct/test_compiler").args(["target/debug/wacc", "--chapter", "1", "--stage", "parse"]).output().unwrap();
    let output = String::from_utf8(output.stderr).unwrap();
    println!("{output}");
    assert!(output.ends_with("OK\n"));
}
