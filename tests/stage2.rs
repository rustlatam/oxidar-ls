#![cfg(feature = "stage2")]

mod helpers;

use helpers::split_ls_output;
use rstest::rstest;
use std::process::Command;

#[rstest]
#[case::short_argument("-a")]
#[case::long_argument("--all")]
fn test_stage_2(#[case] ls_argument: &str) {
    let output = Command::new("./target/debug/oxidar-ls")
        .args([ls_argument, "./test_dir"])
        .output()
        .expect("failed to execute process");

    let expected = Command::new("/bin/ls")
        .args(["-a", "./test_dir"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let expected = split_ls_output(&expected.stdout);
    let output = split_ls_output(&output.stdout);

    assert_eq!(expected, output);
}
