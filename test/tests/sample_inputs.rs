use cli_test_dir::*;

const BIN: &'static str = "./main";

#[test]
fn sample1() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            3
            0 0
            1 2
            2 1
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "2\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample2() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            1
            -691 273
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "0\n");
    assert!(output.stderr_str().is_empty());
}

#[test]
fn sample3() {
    let testdir = TestDir::new(BIN, "");
    let output = testdir
        .cmd()
        .output_with_stdin(
            r#"
            10
            -31 -35
            8 -36
            22 64
            5 73
            -14 8
            18 -58
            -41 -85
            1 -88
            -21 -85
            -11 82
        "#,
        )
        .tee_output()
        .expect_success();
    assert_eq!(output.stdout_str(), "11\n");
    assert!(output.stderr_str().is_empty());
}
