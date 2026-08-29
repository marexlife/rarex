use crate::types::{
    parsed_code::{
        FileNode, FunNode, ItemKind::FuncItem,
        ParsedCode,
    },
    tokens::{
        kind::TokenKind, token_stream::TokenStream,
    },
};

impl ParsedCode {
    pub(crate) fn new(file_node: FileNode) -> Self {
        Self { file_node }
    }
}

impl FunNode {
    pub(crate) fn new() -> Self {
        Self {
            return_type: None,
            args_types: vec![],
            func_body: vec![],
        }
    }

    pub(crate) fn parse(&mut self) {}
}

impl FileNode {
    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }

    pub(crate) fn parse(
        self,
        token_stream: TokenStream,
    ) -> ParsedCode {
        while !token_stream.is_at_end() {
            match *token_stream.kind() {
                TokenKind::Fun => {
                    FunNode::new().parse()
                }
                _ => unreachable!(
                    "not done other func nodes yet"
                ),
            }
        }

        ParsedCode::new(self)
    }
}
