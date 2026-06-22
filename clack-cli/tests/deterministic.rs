use assert_cmd::Command;

#[test]
fn test_deterministic_output() {
    let mut cmd1 = Command::cargo_bin("clack").unwrap();
    let assert1 = cmd1.arg("--seed").arg("42").write_stdin("hello world\n").assert().success();

    let mut cmd2 = Command::cargo_bin("clack").unwrap();
    let assert2 = cmd2.arg("--seed").arg("42").write_stdin("hello world\n").assert().success();

    assert_eq!(assert1.get_output().stdout, assert2.get_output().stdout);
}

#[test]
fn test_different_seeds() {
    let mut cmd1 = Command::cargo_bin("clack").unwrap();
    let assert1 = cmd1.arg("--seed").arg("42").write_stdin("hello world\n").assert().success();

    let mut cmd2 = Command::cargo_bin("clack").unwrap();
    let assert2 = cmd2.arg("--seed").arg("43").write_stdin("hello world\n").assert().success();

    assert_ne!(assert1.get_output().stdout, assert2.get_output().stdout);
}

#[test]
fn test_no_errors_exact_output() {
    let mut cmd = Command::cargo_bin("clack").unwrap();
    let assert = cmd.arg("--no-errors").arg("--seed").arg("42").write_stdin("hello world\n").assert().success();
    assert_eq!(assert.get_output().stdout, b"hello world\n");
}
