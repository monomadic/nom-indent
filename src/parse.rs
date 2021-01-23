use crate::Span;
use nom::{
    bytes::complete::take_while,
    character::complete::{newline, one_of},
    combinator::opt,
    multi::many0_count,
    sequence::tuple,
    IResult,
};
use rctree::Node;

pub(crate) fn indented_tree(i: Span) -> IResult<Span, Vec<Node<Span>>> {
    let (rem, mut lines) = indented_lines(i)?;
    let mut buffer = Vec::new();

    while let Some((indent, line)) = remove_first(&mut lines) {
        let mut node = Node::new(line);
        let children = take_children(&mut lines, indent);

        for child in children {
            node.append(child);
        }
        buffer.push(node);
    }

    Ok((rem, buffer))
}

fn next_indent(lines: &Vec<(usize, Span)>) -> usize {
    *lines.first().map(|(indent, _line)| indent).unwrap_or(&0)
}

fn take_children<'a>(lines: &mut Vec<(usize, Span<'a>)>, indent: usize) -> Vec<Node<Span<'a>>> {
    let mut siblings = Vec::new();

    // while next line is a sibling or child
    while next_indent(&lines) > indent {
        // pop next line
        if let Some((child_indent, line)) = remove_first(lines) {
            let mut node = Node::new(line);

            // take remaining children
            while next_indent(&lines) > child_indent {
                let children = take_children(lines, indent);

                for child in children {
                    node.append(child);
                }
            }

            siblings.push(node);
        }
    }

    siblings
}

fn indented_lines(i: Span) -> IResult<Span, Vec<(usize, Span)>> {
    // note that we cannot use many0 here because nom does not allow
    // the possibility of empty matchers on the many0 combinator.

    let mut rem = i;
    let mut output = Vec::new();

    loop {
        let result = line(rem)?;
        let line = result.1;

        rem = result.0;

        output.push(line);

        // kill the loop when input is consumed
        if rem.to_string().is_empty() {
            break;
        }
    }

    Ok((rem, output))
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
    // return Ok((i, 0));
    many0_count(one_of(" \t"))(i)
}

/// remove first element of a vec. warning: expensive
fn remove_first<T>(vec: &mut Vec<T>) -> Option<T> {
    if vec.is_empty() {
        return None;
    }
    Some(vec.remove(0))
}
