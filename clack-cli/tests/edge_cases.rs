use assert_cmd::Command;

#[test]
fn test_single_character() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--no-errors").write_stdin("a").assert().success();
    assert_eq!(assert.get_output().stdout, b"a");
}

#[test]
fn test_whitespace_only() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--no-errors").write_stdin("   \n\t").assert().success();
    assert_eq!(assert.get_output().stdout, b"   \n\t");
}

#[test]
fn test_wpm_zero_rejection() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    cmd.arg("--wpm").arg("0.0").write_stdin("test").assert().failure();
}

#[test]
fn test_error_rate_zero() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--error-rate").arg("0.0").write_stdin("hello world").assert().success();
    assert_eq!(assert.get_output().stdout, b"hello world");
}

#[test]
fn test_consecutive_sentence_boundaries() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--no-errors").write_stdin("Hello. . . World.").assert().success();
    assert_eq!(assert.get_output().stdout, b"Hello. . . World.");
}

#[test]
fn test_empty_word() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--no-errors").write_stdin("hello  world").assert().success();
    assert_eq!(assert.get_output().stdout, b"hello  world");
}
