use nom::IResult;
use rctree::Node;

mod parse;

/// Span type with location data
pub type Span<'a> = nom_locate::LocatedSpan<&'a str, &'a str>;

/// Return a vector of lines in a string grouped by indent as reference counted nodes.
/// Extra is a `nom-locate` concept for storing additional data such as a filename.
pub fn indent<'a>(i: &'a str, extra: &'a str) -> IResult<Span<'a>, Vec<Node<Span<'a>>>> {
    parse::indented_tree(Span::new_extra(i, extra))
}
