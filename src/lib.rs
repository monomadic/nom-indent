use rctree::Node;
use nom::IResult;
use nom_locate::LocatedSpan;

/// span type with location data
pub type Span<'a> = LocatedSpan<&'a str, &'a str>;

/// return a vector of lines in a string grouped by indent as reference counted nodes.
pub fn indent<'a>(i: &str) -> IResult<Span<'a>, Vec<Node<Span<'a>>>> {
    Ok((Span::new_extra("", ""), Vec::new()))
}
