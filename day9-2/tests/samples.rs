extern crate assert_cli;

#[test]
fn sample1_1() {
    assert_cli::Assert::main_binary()
        .stdin("{}")
        .stdout()
        .is("0")
        .unwrap();
}


#[test]
fn sample1_2() {
    assert_cli::Assert::main_binary()
        .stdin("{{{}}}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample1_3() {
    assert_cli::Assert::main_binary()
        .stdin("{{},{}}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample1_4() {
    assert_cli::Assert::main_binary()
        .stdin("{{{},{},{{}}}}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample1_5() {
    assert_cli::Assert::main_binary()
        .stdin("{<a>,<a>,<a>,<a>}")
        .stdout()
        .is("4")
        .unwrap();
}

#[test]
fn sample1_6() {
    assert_cli::Assert::main_binary()
        .stdin("{{<ab>},{<ab>},{<ab>},{<ab>}}")
        .stdout()
        .is("8")
        .unwrap();
}

#[test]
fn sample1_7() {
    assert_cli::Assert::main_binary()
        .stdin("{{<!!>},{<!!>},{<!!>},{<!!>}}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample1_8() {
    assert_cli::Assert::main_binary()
        .stdin("{{<a!>},{<a!>},{<a!>},{<ab>}}")
        .stdout()
        .is("17")
        .unwrap();
}


#[test]
fn sampl2_0() {
    assert_cli::Assert::main_binary()
        .stdin("{<>}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sampl2_1() {
    assert_cli::Assert::main_binary()
        .stdin("{<random characters>}")
        .stdout()
        .is("17")
        .unwrap();
}

#[test]
fn sampl2_2() {
    assert_cli::Assert::main_binary()
        .stdin("{<<<<>}")
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn sampl2_3() {
    assert_cli::Assert::main_binary()
        .stdin("{<{!>}>}")
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn sampl2_4() {
    assert_cli::Assert::main_binary()
        .stdin("{<!!>}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sampl2_5() {
    assert_cli::Assert::main_binary()
        .stdin("{<!!!>>}")
        .stdout()
        .is("0")
        .unwrap();
}

#[test]
fn sample2_6() {
    assert_cli::Assert::main_binary()
        .stdin("{<{o\"i!a,<{i<a>}")
        .stdout()
        .is("10")
        .unwrap();
}



#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("10045")
        .unwrap();
}
