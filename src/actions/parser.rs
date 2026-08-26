use crate::types::parsed_file::ParsedFile;
use crate::types::token_stream::TokenStream;

#[derive(Debug)]
pub(crate) enum ParserErr {}

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(token_stream: TokenStream) -> Result<ParsedFile, ParserErr> {
        Ok(ParsedFile::new())
    }
}
