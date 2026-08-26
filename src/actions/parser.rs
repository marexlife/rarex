use crate::types::token_stream::TokStream;
use crate::types::parsed_file::ParsedFile;

#[derive(Debug)]
pub(crate) enum ParserErr {}

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(token_stream: TokStream) -> ParsedFile {
        ParsedFile::new()
    }
}
