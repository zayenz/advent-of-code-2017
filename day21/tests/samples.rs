extern crate assert_cli;

#[test]
fn sample1() {
    assert_cli::Assert::main_binary()
        .stdin(
            "\
2
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#",
        )
        .stdout()
        .is("12")
        .unwrap();
}


#[test]
fn puzzle1() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle1.in"))
        .stdout()
        .is("194")
        .unwrap();
}

#[test]
fn puzzle2() {
    assert_cli::Assert::main_binary()
        .stdin(include_str!("../data/puzzle2.in"))
        .stdout()
        .is("2536879")
        .unwrap();
}
