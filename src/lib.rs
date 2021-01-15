use nom::IResult;
use rctree::Node;

mod parse;

/// span type with location data
pub type Span<'a> = nom_locate::LocatedSpan<&'a str, &'a str>;

/// return a vector of lines in a string grouped by indent as reference counted nodes.
pub fn indent<'a>(i: &'a str, extra: &'a str) -> IResult<Span<'a>, Vec<Node<Span<'a>>>> {
    // Ok((Span::new_extra(i, extra), Vec::new()))
    parse::lines(Span::new_extra(i, extra))
}
