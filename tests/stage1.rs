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
    assert_eq!(output.stdout, expected.stdout);
}
