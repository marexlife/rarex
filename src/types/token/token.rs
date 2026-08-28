use crate::types::token::{
    kind::TokenKind, pos::TokenPos,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct Token {
    kind: TokenKind,
    pos: TokenPos,
}

impl Token {
    pub(crate) fn new(
        kind: TokenKind,
        pos: TokenPos,
    ) -> Self {
        Self { kind, pos }
    }

    pub(crate) fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub(crate) fn pos(&self) -> TokenPos {
        self.pos
    }
}
