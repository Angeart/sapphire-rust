use crate::parser::utils::sign;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::{
        complete::{char, digit1},
        is_hex_digit,
    },
    combinator::{complete, cut, map, map_res, opt, recognize},
    error::{ErrorKind, FromExternalError, ParseError},
    sequence::tuple,
    AsChar, Compare, IResult, InputIter, InputTakeAtPosition,
};
use std::i32;

use super::traits::ParserInput;

pub type Integer = i32;
pub type Float = f32;

#[derive(PartialEq, Debug)]
pub enum Number {
    Integer(Integer),
    Float(Float),
}

pub fn integer32_value<'a, I, E: ParseError<I>>(s: I) -> IResult<I, Integer, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
    E: FromExternalError<I, anyhow::Error>,
{
    map_res(
        recognize(tuple((sign, digit1))),
        |value: I| -> Result<Integer, anyhow::Error> {
            let value = value.as_str().parse::<Integer>()?;
            Ok(value)
        },
    )(s)
}

pub fn hex_integer32_value<'a, I, E: ParseError<I>>(s: I) -> IResult<I, Integer, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar,
    <I as InputTakeAtPosition>::Item: AsChar,
    E: FromExternalError<I, anyhow::Error>,
{
    map_res(
        complete(tuple((
            tag("0x"),
            take_while(|x: <I as InputTakeAtPosition>::Item| is_hex_digit(x.as_char() as u8)),
        ))),
        |(_, v): (I, I)| -> Result<Integer, anyhow::Error> {
            let value = Integer::from_str_radix(v.as_str(), 16)?;
            Ok(value)
        },
    )(s)
}

pub fn float32_value<'a, I, E: ParseError<I>>(s: I) -> IResult<I, Float, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
    E: FromExternalError<I, anyhow::Error>,
{
    map_res(
        recognize(tuple((
            sign,
            alt((
                map(
                    tuple((
                        alt((
                            map(tuple((digit1, char('.'), opt(digit1))), |_| ()),
                            map(tuple((char('.'), digit1)), |_| ()),
                        )),
                        opt(tuple((alt((char('e'), char('E'))), sign, cut(digit1)))),
                    )),
                    |_| (),
                ),
                map(
                    tuple((
                        digit1,
                        tuple((alt((char('e'), char('E'))), sign, cut(digit1))),
                    )),
                    |_| (),
                ),
            )),
        ))),
        |value: I| -> Result<Float, anyhow::Error> {
            let value = value.as_str().parse::<Float>()?;
            Ok(value)
        },
    )(s)
}

pub fn number<'a, I, E: ParseError<I>>(s: I) -> IResult<I, Number, E>
where
    I: ParserInput<'a>,
    <I as InputIter>::Item: AsChar + Copy,
    <I as InputTakeAtPosition>::Item: AsChar,
    E: FromExternalError<I, anyhow::Error>,
{
    alt((
        map(hex_integer32_value, Number::Integer),
        map(float32_value, Number::Float),
        map(integer32_value, Number::Integer),
    ))(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_parse;
    use nom::Err;
    #[test]
    fn interger32_value_test() {
        assert_parse!(integer32_value("1"), Ok(("", 1)));
        assert_parse!(integer32_value("+20"), Ok(("", 20)));
        assert_parse!(integer32_value("-114"), Ok(("", -114)));
        // This test will be success
        assert_parse!(integer32_value("1.1"), Ok((".1", 1)));
    }

    #[test]
    fn hex_integer32_value_test() {
        assert_parse!(hex_integer32_value("0x1a2A"), Ok(("", 0x1a2a)));
        assert_parse!(hex_integer32_value("0x1x2"), Ok(("x2", 1)));
    }

    #[test]
    fn float32_value_test() {
        assert_parse!(float32_value("1.1"), Ok(("", 1.1)));
        assert_parse!(float32_value(".23"), Ok(("", 0.23)));
        assert_parse!(float32_value("1.2e-8"), Ok(("", 1.2e-8)));
        assert_parse!(
            float32_value("efg"),
            Err(Err::Error(("efg", ErrorKind::Digit)))
        );
        assert_parse!(float32_value("111"), Err(Err::Error(("", ErrorKind::Char))));
    }

    #[test]
    fn number_test() {
        assert_parse!(
            number("0xab"),
            Ok(("", Number::Integer(v))) if v == 0xab
        );
        assert_parse!(
            number("11"),
            Ok(("", Number::Integer(v))) if v == 11
        );
        assert_parse!(
            number("1.1"),
            Ok(("", Number::Float(v))) if v == 1.1
        );
    }
}
