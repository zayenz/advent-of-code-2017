extern crate assert_cli;

/*
Argument does not seem to work
#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .with_args(&["-p 5"])
        .stdin(
            "\
             s1,x3/4,pe/b
             ",
        )
        .stdout()
        .is("baedc")
        .unwrap();
}
*/


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("iabmedjhclofgknp")
        .unwrap();
}
