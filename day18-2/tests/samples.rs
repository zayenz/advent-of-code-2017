extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d",
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
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2",
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
        .is("5969")
        .unwrap();
}
