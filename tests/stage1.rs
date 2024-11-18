#![cfg(feature = "stage1")]

mod helpers;

use helpers::split_ls_output;
use std::process::Command;

#[test]
fn test_stage_1() {
    let output = Command::new("./target/debug/oxidar-ls")
        .args(["./test_dir"])
        .output()
        .expect("failed to execute process");

    let expected = Command::new("/bin/ls")
        .args(["./test_dir"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let expected = split_ls_output(&expected.stdout);
    let output = split_ls_output(&output.stdout);

    assert_eq!(expected, output);
}
