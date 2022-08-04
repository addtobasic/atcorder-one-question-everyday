use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            2
            2 -1
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "3
2.236067977499790
2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            10
            3 -1 -4 1 -5 9 2 -6 5 -3
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "39
14.387494569938159
9\n");
    assert!(output.stderr_str().is_empty());
}
