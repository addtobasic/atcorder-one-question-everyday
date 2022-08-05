use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            -9 99 -999 9999
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "-18.7058823529\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1 1 7 2
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3.0000000000\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1 1 3 2
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "1.6666666667\n");
    assert!(output.stderr_str().is_empty());
}
