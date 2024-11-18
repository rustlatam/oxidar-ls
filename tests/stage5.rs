#![cfg(feature = "stage5")]

mod helpers;

use helpers::{split_ls_column_output, split_ls_output_by_newline};
use rstest::rstest;
use std::process::Command;

#[rstest]
#[case::arguments_l(&["-l"])]
#[case::arguments_lF(&["-l", "-F"])]
#[case::arguments_lFh(&["-l", "-F", "-h"])]
#[case::arguments_lFha(&["-l", "-F", "-h", "-a"])]
#[case::arguments_lFha(&["-lFha"])] // This tests tha clap was introduced and works as expected
fn test_stage_5(#[case] ls_arguments: &[&'static str]) {
    let output = Command::new("./target/debug/oxidar-ls")
        .args([ls_arguments, &["./test_dir"]].concat())
        .output()
        .expect("failed to execute process");

    let expected = Command::new("/bin/ls")
        .args([ls_arguments, &["./test_dir"]].concat())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    let expected_lines = split_ls_output_by_newline(&expected.stdout);
    let output_lines = split_ls_output_by_newline(&output.stdout);

    // We use expected_lines[1..] because the first element in the /bin/ls output is the total
    // column
    assert_eq!(expected_lines[1..].len(), output_lines.len());

    for (expected, output) in expected_lines[1..].iter().zip(output_lines) {
        let expected = split_ls_column_output(*expected);
        let output = split_ls_column_output(&*output);
        // Assert permissions
        assert_eq!(expected[0], output[0]);

        // We ignore the . and ../ directories on purpose because is not that easy to calculate
        // their size
        if output[2] != "." && output[2] != "./" && output[2] != ".." && output[2] != "../" {
            assert_eq!(expected[4], output[1]);
        }
        // Assert name
        assert_eq!(expected[8], output[2]);
    }
}
