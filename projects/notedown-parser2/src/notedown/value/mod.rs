use nom::error::ParseError;
use nom::combinator::value;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::branch::alt;
use nom_locate::{LocatedSpan, position};
use notedown_ast::{AST, Value};


type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
struct Token<'a> {
    pub position: Span<'a>,
    pub rest: &'a str,
}


fn parse_boolean(s: Span) -> IResult<Span, Value> {
    let (rest, pat ) = alt((tag("true"), tag("false")))(s)?;

    println!("{:?}",pat);
    let (s, pos) = position(pat)?;
    let token = Value::Boolean {
        position: pos,
        rest: s.fragment()
    };
    println!("{:?}",rest);
    println!("{:?}",token);

    Ok((
        rest,
        token,
    ))
}

#[test]
fn main() {
    let input = Span::new("**true is bold**");
    parse_boolean(input);
}
