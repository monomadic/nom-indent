use nom_indent::*;

fn assert_number_of_lines(test: &str, lines: usize) {
    match indent(test, "<assertion>") {
        Ok(result) => assert_eq!(result.1.len(), lines),
        Err(e) => panic!("statement failed to parse {}, {}", test, e),
    }
}

/// a small render syntax for simple testing
fn assert_mini_parser(input: &str, output: &str) {
    use rctree::Node;

    fn bracketer(node: Node<Span>, buffer: &mut String) {
        buffer.push_str(&node.borrow().to_string());
    }

    let mut buffer = String::new();

    // parse
    let result = indent(input, "<assertion>").expect("input failed to parse");
    for node in result.1 {
        buffer.push_str("(");
        bracketer(node, &mut buffer);
        buffer.push_str(")");
    }

    println!("---{}", buffer);
}

#[test]
fn test_parser() {
    assert_number_of_lines("", 1);
    assert_number_of_lines("\n", 1);
    assert_number_of_lines("\n\n", 2);

    assert_number_of_lines("a\nb\nc", 3);

    assert_mini_parser("", "()");
    assert_mini_parser("a", "(a)");
    assert_mini_parser("a\nb", "(a)(b)");
}
