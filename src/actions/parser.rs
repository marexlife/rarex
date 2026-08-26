use crate::tok_stream::TokStream;

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(token_stream: TokStream) -> ParsedFile {
        ParsedFile::new()
    }
}
