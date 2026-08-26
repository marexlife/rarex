use crate::types::tok_stream::TokStream;
use crate::types::tok_stream::TokStream;

#[derive(Debug)]
pub(crate) enum ParserErr {}

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(token_stream: TokStream) -> ParsedFile {
        ParsedFile::new()
    }
}
