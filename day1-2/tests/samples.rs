extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("1212")
        .stdout().is("6")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("1221")
        .stdout().is("0")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("123425")
        .stdout().is("4")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin("123123")
        .stdout().is("12")
        .unwrap();
}

#[test]
fn sample5() {
    assert_cli::Assert::main_binary()
        .stdin("12131415")
        .stdout().is("4")
        .unwrap();
}
