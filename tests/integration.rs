use nom_indent::*;
// use nom_locate::position;

fn assert_number_of_lines(test: &str, lines: usize) {
    match indent(test, "<assertion>") {
        Ok(result) => assert_eq!(result.1.len(), lines),
        Err(e) => panic!("statement failed to parse {}, {}", test, e),
    }
}

#[test]
fn test_parser() {
    assert_number_of_lines("", 1);
    assert_number_of_lines("\n", 1);
    assert_number_of_lines("\n\n", 2);

    assert_number_of_lines("a\nb\nc", 3);
}
