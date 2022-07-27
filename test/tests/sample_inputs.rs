use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            200 300

        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "200\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            10000 0

        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "20100\n");
    assert!(output.stderr_str().is_empty());
}
