use super::traits::ParserInput;
use nom::branch::{alt, permutation};
use nom::bytes::complete::{escaped_transform, take_while_m_n};
use nom::character::complete::{char, none_of};
use nom::combinator::{map, value};
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;
use nom::*;
use nom_greedyerror::AsStr;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::ops::RangeFrom;
use std::u16;

pub fn string_value<'a, I, E: ParseError<I>>(s: I) -> IResult<I, String, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as ExtendInto>::Item: AsChar + Copy,
    &'a str: FindToken<<I as InputIter>::Item>,
{
    delimited(
        char('\"'),
        escaped_transform(
            none_of("\"\\"),
            '\\',
            alt((
                value('\\', char('\\')),
                value('\"', char('\"')),
                value('\'', char('\'')),
                value('\r', char('r')),
                value('\n', char('n')),
                value('\t', char('t')),
                map(
                    permutation((
                        char('u'),
                        take_while_m_n(4, 4, |c: <I as InputIter>::Item| {
                            c.as_char().is_ascii_hexdigit()
                        }),
                    )),
                    |(_, code): (char, I)| -> char {
                        decode_utf16(vec![u16::from_str_radix(code.as_str(), 16).unwrap()])
                            .nth(0)
                            .unwrap()
                            .unwrap_or(REPLACEMENT_CHARACTER)
                    },
                ),
            )),
        ),
        char('\"'),
    )(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use nom::{error::ErrorKind, Err};
    #[test]
    fn string_value_test() {
        assert_parse!(
            string_value("\"a\\\"b\\\'c\""),
            Ok(("", String::from("a\"b\'c")))
        );
        assert_parse!(
            string_value("\" \\r\\n\\t \\u2615 \\uDD1E\""),
            Ok(("", String::from(" \r\n\t ☕ �")))
        );
        assert_eq!(
            string_value("\"abc"),
            Err(Err::Error(("", ErrorKind::Char)))
        );
        assert_eq!(
            string_value("\"ab\\zc\""),
            Err(Err::Error(("zc\"", ErrorKind::TakeWhileMN)))
        );
    }
}
