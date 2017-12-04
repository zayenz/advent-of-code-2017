extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("1")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("12")
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("23")
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin("1024")
        .stdout()
        .is("31")
        .unwrap();
}

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin("289326")
        .stdout()
        .is("419")
        .unwrap();
}
