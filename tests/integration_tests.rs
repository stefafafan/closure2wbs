use assert_cmd::Command;

#[test]
fn version() {
    let mut cmd = Command::cargo_bin("closure2wbs").unwrap();
    cmd.arg("-V")
        .assert()
        .success()
        .stdout("closure2wbs 0.3.0\n");
}
