use nom_indent::*;
use nom_locate::position;

fn assert_lines(test: &str, lines: usize) {
    match indent(test, "") {
        Ok(result) => assert_eq!(result.0.len(), lines),
        Err(_) => panic!("statement failed to parse {}.", test),
    }
}

#[test]
fn test_parser() {
    assert_lines("", 0);
}
