use nom_indent::*;

/// a small render syntax for simple testing
fn assert_mini_parser(input: &str, output: &str) {
    use rctree::Node;

    fn bracketer(node: &Node<Span>, buffer: &mut String) {
        buffer.push_str("(");
        buffer.push_str(&node.borrow().to_string());
        for child in node.children() {
            bracketer(&child, buffer);
        }
        buffer.push_str(")");
    }

    // parse
    let (_rem, lines) = indent(input, "<assertion>").expect("input failed to parse");

    let mut buffer = String::new();
    for line in lines {
        bracketer(&line, &mut buffer);
    }

    assert_eq!(buffer, output);
}

#[test]
fn test_parser() {
    assert_mini_parser("", "()");
    assert_mini_parser("a", "(a)");
    assert_mini_parser("a\nb", "(a)(b)");
    assert_mini_parser("a\n\tb", "(a(b))");
    assert_mini_parser("a\n\tb\n\t\tc", "(a(b(c)))");
    assert_mini_parser("a\n\tb\nc", "(a(b))(c)");
    assert_mini_parser("a\n\tb\n\tc\nd", "(a(b)(c))(d)");
    assert_mini_parser("a\n\tb\n\t\tc\n\td", "(a(b(c)(d)))");
}
