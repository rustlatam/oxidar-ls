#![cfg(feature = "stage1")]

use std::process::Command;

#[test]
fn test_stage1() {
    let output = Command::new("./target/debug/oxidar-ls")
        .args(["-a", "./test_dir"])
        .output()
        .expect("failed to execute process");

    let expected = Command::new("/bin/ls")
        .args(["-a", "./test_dir"])
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(expected.stdout).unwrap());
}
