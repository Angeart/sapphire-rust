use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{map, opt, recognize},
    error::ParseError,
    multi::many0,
    sequence::{delimited, pair},
    IResult, *,
};

use super::traits::ParserInput;
use crate::token::TokenKind;

pub fn sign<'a, I, E: ParseError<I>>(input: I) -> IResult<I, Option<I>, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar + Copy,
{
    opt(alt((
        tag(TokenKind::Plus.token()),
        tag(TokenKind::Minus.token()),
    )))(input)
}

pub fn identifier<'a, I, E: ParseError<I>>(input: I) -> IResult<I, String, E>
where
    I: ParserInput<'a> + Compare<&'a str>,
    <I as InputTakeAtPosition>::Item: AsChar,
{
    map(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |x: I| String::from(x.as_str()),
    )(input)
}

pub fn ws<'a, I, F, O, E: ParseError<I>>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: ParserInput<'a>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    F: Fn(I) -> IResult<I, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn sign_test() {
        assert_parse!(sign(TokenKind::Plus.token()), Ok(("", Some(v))) if v == TokenKind::Plus.token());
        assert_parse!(sign(TokenKind::Minus.token()), Ok(("", Some(v))) if v == TokenKind::Minus.token());
        assert_parse!(sign(""), Ok(("", None)));
    }

    #[test]
    fn identifier_test() {
        assert_parse!(identifier("hoge"), Ok(("", "hoge".into())));
        assert_parse!(
            identifier("_ThisIs_Identifier2"),
            Ok(("", "_ThisIs_Identifier2".into()))
        );
        assert_parse!(
            identifier("2foo"),
            Err(Err::Error(("2foo", ErrorKind::Tag)))
        );
    }

    #[test]
    fn ws_test() {
        assert_parse!(ws(alpha1)(" A"), Ok(("", "A")));
        assert_parse!(ws(alpha1)("A "), Ok(("", "A")));
    }
}
