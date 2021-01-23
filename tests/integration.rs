use nom_indent::*;

// fn assert_number_of_lines(test: &str, lines: usize) {
//     match indent(test, "<assertion>") {
//         Ok(result) => {
//             let mut count = 0;

//             for child in result.1.children() {
//                 count = count + 1;
//             }

//             assert_eq!(count, lines)
//         }
//         Err(e) => panic!("statement failed to parse {}, {}", test, e),
//     }
// }

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
