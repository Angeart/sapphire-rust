use crate::symbol_repr::SymbolRepl;

use nom_locate::LocatedSpan;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
pub struct SourceLocation<'a> {
    pub span: SymbolRepl<LocatedSpan<&'a str>>,
}

// #[derive(Debug, Clone)]
// pub struct SourceLocationTable<'a> {}

#[cfg(test)]
mod tests {
    use nom::error::ErrorKind;
    use nom::Err;
    use nom_greedyerror::error_position;
    use nom_greedyerror::GreedyError;

    use super::*;
    // use crate::assert_parse;
    use crate::parser::utils::identifier;
    use nom::combinator::eof;
    use nom::sequence::pair;
    #[test]
    fn integration_test() {
        let error = pair::<_, _, _, GreedyError<LocatedSpan<&str>, ErrorKind>, _, _>(
            identifier, eof,
        )(LocatedSpan::from("hoge//fuga"));
        match error {
            Err(Err::Error(e)) => assert_eq!(error_position(&e), Some(4)),
            _ => assert!(false),
        }
    }
}
