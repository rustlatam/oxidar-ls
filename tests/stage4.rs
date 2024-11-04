#![cfg(feature = "stage4")]

mod helpers;

use rstest::rstest;
use std::process::Command;

const EXPECTED_HELP_TEXT: &str = r#"A tiny version of ls command

Usage: oxidar-ls [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to list

Options:
  -a, --all     Show hidden files
  -F, --format  Format file names. Appendds a symbol at the end indicating the type of file. Slash ('/') is for directories and at sign ('@') is for symbolic links
  -l, --list    Show the files in a list
  -h, --human   Show the file size in a human-redable format
      --help
"#;

#[rstest]
#[case::arguments_help(&["--help"])]
fn test_stage_4(#[case] ls_arguments: &[&'static str]) {
    let output = Command::new("./target/debug/oxidar-ls")
        .args([ls_arguments, &["./test_dir"]].concat())
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());

    assert_eq!(
        EXPECTED_HELP_TEXT,
        std::str::from_utf8(&output.stdout).unwrap()
    );
}
