extern crate assert_cli;

#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle_optimized.in"))
        .stdout()
        .is("907")
        .unwrap();
}
