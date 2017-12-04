extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             abcde fghij
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
             abcde xyz ecdab
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
             a ab abc abd abf abj
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
             iiii oiii ooii oooi oooo
             ",
        )
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn sample5() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             oiii ioii iioi iiio
             ",
        )
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample6() {
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
        .is("119")
        .unwrap();
}
