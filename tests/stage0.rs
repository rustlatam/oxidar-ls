use std::process::Command;

#[test]
fn test_stage0() {
    let output = Command::new("./target/debug/oxidar-ls")
        .args(["./test_dir"])
        .output()
        .expect("failed to execute process");

    let expected = Command::new("/bin/ls")
        .args(["./test_dir"])
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(expected.stdout).unwrap());
}
