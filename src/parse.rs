use crate::Span;
use nom::{
    bytes::complete::take_while,
    character::complete::{alphanumeric0, alphanumeric1, newline, one_of},
    combinator::{cut, opt},
    multi::{many0, many0_count, many1, many1_count},
    sequence::tuple,
    IResult,
};
use rctree::Node;

/// break multiple lines
pub(crate) fn lines(i: Span) -> IResult<Span, Vec<Node<Span>>> {
    // note that we cannot use many0 here because nom does not allow
    // the possibility of empty matchers on the many0 combinator.

    let mut rem = i;
    let mut output = Vec::new();

    loop {
        let result = line(rem)?;
        let (_indent, content) = result.1;
        rem = result.0;
        output.push(Node::new(content));

        // kill the loop when input is consumed
        if rem.to_string().is_empty() {
            break;
        }
    }

    println!("returning {:?}", (rem, output.clone()));

    Ok((rem, output))

    // many0(line)(i).map(|(r, lines)| {
    //     (
    //         r,
    //         lines
    //             .into_iter()
    //             .map(|(_line_count, line_content)| line_content)
    //             .collect(),
    //     )
    // })
}

/// take a single line in the format (indent, content) and chomp optional newline
pub(crate) fn line(i: Span) -> IResult<Span, (usize, Span)> {
    if i.to_string() == "" {
        return Ok((i, (0, i)));
    }
    tuple((
        // opt(many0(tuple((space0, newline)))), // throw away blank lines
        count_indentation,
        until_newline_or_eof,
        // alt((eof, newline))
        // is_not("\n"),
        opt(newline),
    ))(i)
    .map(|(r, (indent, line, _))| (r, (indent, line)))
}

/// match an input until either a newline or end of file is found
fn until_newline_or_eof(i: Span) -> IResult<Span, Span> {
    take_while(|c| c != '\n')(i)
}

fn count_indentation(i: Span) -> IResult<Span, usize> {
    return Ok((i, 0));
    // many1_count(one_of(" \t"))(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lines() {
        println!("-- {:?}", lines(Span::new_extra("", "")));
        assert!(lines(Span::new_extra("", "")).is_ok());
        assert!(lines(Span::new_extra("\n", "")).is_ok());
        assert!(line(Span::new_extra(" ", "")).is_ok());
        assert!(line(Span::new_extra(" \n", "")).is_ok());
    }

    #[test]
    fn test_line() {
        assert!(line(Span::new_extra("", "")).is_ok());
        assert!(line(Span::new_extra("\n", "")).is_ok());
        assert!(line(Span::new_extra(" ", "")).is_ok());
        assert!(line(Span::new_extra(" \n", "")).is_ok());
    }

    #[test]
    fn test_count_indentation() {
        assert!(count_indentation(Span::new_extra("", "")).is_ok(), 0);
        assert!(count_indentation(Span::new_extra("\n", "")).is_ok(), 0);
        assert!(count_indentation(Span::new_extra("aa bb", "")).is_ok(), 0);
        assert!(count_indentation(Span::new_extra("\ta\tb", "")).is_ok(), 1);
        assert!(count_indentation(Span::new_extra("\t\t", "")).is_ok(), 2);
        assert!(count_indentation(Span::new_extra(" ", "")).is_ok(), 1);
        assert!(count_indentation(Span::new_extra("\t", "")).is_ok(), 1);
        assert!(count_indentation(Span::new_extra("\t ", "")).is_ok(), 2);
        assert!(count_indentation(Span::new_extra("\t \t", "")).is_ok(), 3);
    }
}
