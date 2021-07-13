use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    combinator::value,
    error::{ErrorKind, ParseError},
    sequence::{pair, tuple},
    IResult,
};

pub fn oneline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(tag("//"), is_not("\n\r")),
    )(i)
}

pub fn multiline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        tuple((tag("/*"), take_until("*/"), tag("*/"))),
    )(i)
}

pub fn comment(s: &str) -> IResult<&str, (), (&str, ErrorKind)> {
    alt((oneline_comment, multiline_comment))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    #[test]
    fn oneline_comment_test() {
        assert_eq!(
            oneline_comment::<(_, ErrorKind)>("// this is comment"),
            Ok(("", ()))
        );
    }
    #[test]
    fn multiline_comment_test() {
        assert_eq!(
            multiline_comment::<(_, ErrorKind)>("/* this is comment \n */"),
            Ok(("", ()))
        );
    }
}
