# nom-indent

Significant whitespace indentation parser for `nom` with `nom-locate` span support.

This is a surprisingly hard problem with no readily available solutions, so I have made a quick crate. It was written for use in the `astryx programming language`, but could be adapted to any whitespace significant indented syntax.

It is designed to just break up your input into a node tree of spans and then get out of your way, allowing you to send line data to your parser from there. It is agnostic to internal syntax other than indentation and linebreaks.
