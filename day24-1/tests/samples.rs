extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10",
        )
        .stdout()
        .is("31")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("1511")
        .unwrap();
}
