use assert_cmd::Command;
use httpmock::prelude::*;
use std::fs;
use std::path::Path;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage"));
    Ok(())
}

#[test]
fn downloads_font_files() -> TestResult {
    let server = MockServer::start();
    let css_request_mock = server.mock(|when, then| {
        when.method("GET")
            .path("/css")
            .query_param("family", "Roboto:300,300i,400");
        then.status(200)
            .body_from_file("tests/test_files/example.css");
    });

    let woff_i3x_mock = server.mock(|when, then| {
        when.method("GET").path("/ri3lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--italic--300--latin-ext.woff2");
    });

    let woff_i3_mock = server.mock(|when, then| {
        when.method("GET").path("/ri3l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--italic--300--latin.woff2");
    });

    let woff_n3x_mock = server.mock(|when, then| {
        when.method("GET").path("/rn3lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--300--latin-ext.woff2");
    });

    let woff_n3_mock = server.mock(|when, then| {
        when.method("GET").path("/rn3l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--300--latin.woff2");
    });

    let woff_n4x_mock = server.mock(|when, then| {
        when.method("GET").path("/rn4lx.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--400--latin-ext.woff2");
    });

    let woff_n4_mock = server.mock(|when, then| {
        when.method("GET").path("/rn4l.woff2");
        then.status(200)
            .body_from_file("tests/test_files/Roboto--normal--400--latin.woff2");
    });

    let css_url = server.url("/css?family=Roboto:300,300i,400");
    let dir = Path::new("./tmp");

    // Remove the output directory if it already exists
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }

    let mut cmd = Command::cargo_bin("shfonts")?;
    cmd.arg(css_url)
        .arg(format!("--dir={}", dir.to_str().unwrap()))
        .assert()
        .success();

    // Check if server requests were called
    css_request_mock.assert();
    woff_i3x_mock.assert();
    woff_i3_mock.assert();
    woff_n3x_mock.assert();
    woff_n3_mock.assert();
    woff_n4x_mock.assert();
    woff_n4_mock.assert();

    // Check that the font files were downloaded correctly
    assert!(dir.join("ri3lx.woff2").exists());
    assert!(dir.join("ri3l.woff2").exists());
    assert!(dir.join("rn3lx.woff2").exists());
    assert!(dir.join("rn3l.woff2").exists());
    assert!(dir.join("rn4lx.woff2").exists());
    assert!(dir.join("rn4l.woff2").exists());

    Ok(())
}
