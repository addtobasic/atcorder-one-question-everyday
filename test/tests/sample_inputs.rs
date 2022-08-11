use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            10 2 20
            9 11
            13 17
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            10 2 20
            9 11
            13 16
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            15 3 30
            5 8
            15 17
            24 27
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "Yes\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample4() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            20 1 30
            20 29
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample5() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            20 1 30
            1 10
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "No\n");
    assert!(output.stderr_str().is_empty());
}
