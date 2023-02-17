use assert_cmd::Command;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}
