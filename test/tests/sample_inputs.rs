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
            1 3
            3 5
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "18\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            3
            11 13
            17 47
            359 44683
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "998244353\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1
            1 1000000
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "500000500000\n");
    assert!(output.stderr_str().is_empty());
}
