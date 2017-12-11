extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             ne,ne,ne
             ",
        )
        .stdout()
        .is("3")
        .unwrap();
}

#[test]
fn sample2() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             ne,ne,sw,sw
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
             ne,ne,s,s
             ",
        )
        .stdout()
        .is("2")
        .unwrap();
}

#[test]
fn sample4() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
             se,sw,se,sw,sw
             ",
        )
        .stdout()
        .is("3")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("707")
        .unwrap();
}
