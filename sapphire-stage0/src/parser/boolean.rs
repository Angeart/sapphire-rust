use nom::{branch::alt, bytes::complete::tag, combinator::value, error::ErrorKind, IResult};
pub fn boolean<'a>(input: &'a str) -> IResult<&'a str, bool, (&str, ErrorKind)> {
    alt((value(true, tag("true")), value(false, tag("false"))))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;
    #[test]
    fn boolean_test() {
        assert_eq!(boolean("true"), Ok(("", true)));
        assert_eq!(boolean("false"), Ok(("", false)));
        assert_eq!(boolean("hoge"), Err(Err::Error(("hoge", ErrorKind::Tag))))
    }
}
