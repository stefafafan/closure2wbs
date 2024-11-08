use assert_cmd::Command;
use std::fs;

#[test]
fn plantuml() {
    let mut cmd = Command::cargo_bin("closure2wbs").unwrap();
    cmd.arg("--input")
        .arg("tests/fixtures/input.json")
        .arg("--output")
        .arg("tests/fixtures/output.puml")
        .arg("--format")
        .arg("plantuml")
        .assert()
        .success();

    let output_contents =
        fs::read_to_string("tests/fixtures/output.puml").expect("Unable to read output file");

    let expected_output =
        "@startwbs\n* A\n** B\n*** C\n**** F\n**** G\n*** D\n**** H\n*** E\n**** I\n@endwbs\n";

    assert_eq!(output_contents, expected_output);

    fs::remove_file("tests/fixtures/output.puml").expect("Unable to remove output file");
}

#[test]
fn mermaid() {
    let mut cmd = Command::cargo_bin("closure2wbs").unwrap();
    cmd.arg("--input")
        .arg("tests/fixtures/input.json")
        .arg("--output")
        .arg("tests/fixtures/output.mmd")
        .arg("--format")
        .arg("mermaid")
        .assert()
        .success();

    let output_contents =
        fs::read_to_string("tests/fixtures/output.mmd").expect("Unable to read output file");

    let expected_output =
        "flowchart TD\nA --> B\nB --> C\nC --> F\nC --> G\nB --> D\nD --> H\nB --> E\nE --> I\n";

    assert_eq!(output_contents, expected_output);

    fs::remove_file("tests/fixtures/output.mmd").expect("Unable to remove output file");
}
