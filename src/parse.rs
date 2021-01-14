use crate::Span;
use nom::{
    bytes::complete::is_not,
    character::complete::{newline, one_of, space0},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

/// take a single line in the format (indent, content) and chomp newline
pub(crate) fn line(i: Span) -> IResult<Span, (usize, Span)> {
    tuple((
        opt(many0(tuple((space0, newline)))), // throw away blank lines
        nom::multi::many0_count(one_of(" \t")),
        is_not("\n"),
        opt(newline),
    ))(i)
    .map(|(r, (_, indent, line, _))| (r, (indent, Span::new_extra(line.fragment(), "whoops"))))
}
