use crate::types::{
    parsed_code::ParsedCode, tokens::token_stream::TokenStream,
};

pub(crate) fn parse(token_stream: TokenStream) {
    let mut parsed_code = ParsedCode::new();

    while !token_stream.is_at_end() {}
}
