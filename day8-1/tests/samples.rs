extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
             ",
        )
        .stdout()
        .is("1")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("4448")
        .unwrap();
}
