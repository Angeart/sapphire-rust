use derive_builder::Builder;
use std::ffi::OsStr;
use std::{fs, path};

pub mod boolean;
pub mod comment;
pub mod number;
pub mod string;
pub mod traits;
pub mod utils;
trait ParserTrait {
    fn open<T: AsRef<OsStr> + ?Sized>(v: &T) -> anyhow::Result<Parser>;
    fn parse(self) -> String;
}

#[derive(Default, Builder, Debug)]
pub struct Parser {
    source: String,
}

impl ParserTrait for Parser {
    fn open<T: AsRef<OsStr> + ?Sized>(v: &T) -> anyhow::Result<Parser> {
        let path = path::Path::new(v);
        let content = fs::read_to_string(path)?;
        let parser = ParserBuilder::default().source(content).build()?;
        Ok(parser)
    }
    fn parse(self) -> std::string::String {
        todo!()
    }
}
