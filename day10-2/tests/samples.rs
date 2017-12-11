extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin("")
        .stdout()
        .is("a2582a3a0e66e6e86e3812dcb672a272")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin("AoC 2017")
        .stdout()
        .is("33efeb34ea91902bb2f59c9920caa6cd")
        .unwrap();
}

#[test]
fn sample3() {
    assert_cli::Assert::main_binary()
        .stdin("1,2,3")
        .stdout()
        .is("3efbe78a8d82f29979031a4aa0b16a9d")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin("1,2,4")
        .stdout()
        .is("63960835bcdc130f0b66d7ff4f6a5a8e")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("decdf7d377879877173b7f2fb131cf1b")
        .unwrap();
}
