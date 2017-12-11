extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("{}")
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("{{{}}}")
        .stdout()
        .is("6")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("{{},{}}")
        .stdout()
        .is("5")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin("{{{},{},{{}}}}")
        .stdout()
        .is("16")
        .unwrap();
}

#[test]
fn sample5() {
    assert_cli::Assert::main_binary()
        .stdin("{<a>,<a>,<a>,<a>}")
        .stdout()
        .is("1")
        .unwrap();
}

#[test]
fn sample6() {
    assert_cli::Assert::main_binary()
        .stdin("{{<ab>},{<ab>},{<ab>},{<ab>}}")
        .stdout()
        .is("9")
        .unwrap();
}

#[test]
fn sample7() {
    assert_cli::Assert::main_binary()
        .stdin("{{<!!>},{<!!>},{<!!>},{<!!>}}")
        .stdout()
        .is("9")
        .unwrap();
}

#[test]
fn sample8() {
    assert_cli::Assert::main_binary()
        .stdin("{{<a!>},{<a!>},{<a!>},{<ab>}}")
        .stdout()
        .is("3")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("23588")
        .unwrap();
}
