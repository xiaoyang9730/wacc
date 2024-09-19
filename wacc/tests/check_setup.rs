use std::process::Command;

#[test]
fn check_setup() {
    let output = Command::new("../wacct/test_compiler").arg("--check-setup").output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    assert_eq!(output, "All system requirements met!\n");
}
