use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            100 200 2
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "10 20\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            120 150 2
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "14 16\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            300 333 1
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "UNSATISFIABLE\n");
    assert!(output.stderr_str().is_empty());
}
