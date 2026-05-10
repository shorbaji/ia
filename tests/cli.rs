use std::process::Command;

#[test]
fn prints_hello_world() {
    let output = Command::new(env!("CARGO_BIN_EXE_ia"))
        .output()
        .expect("failed to run ia");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");
    assert_eq!(stdout.trim(), "hello, world!");
}
