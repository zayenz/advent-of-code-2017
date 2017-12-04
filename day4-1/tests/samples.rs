extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             aa bb cc dd ee
             ",
        )
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             aa bb cc dd aa
             ",
        )
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             aa bb cc dd aaa
             ",
        )
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             aa bb cc dd ee
             aa bb cc dd aa
             aa bb cc dd aaa
             ",
        )
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("325")
        .unwrap();
}
