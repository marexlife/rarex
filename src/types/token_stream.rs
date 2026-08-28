use crate::types::token::{
    kind::TokenKind, pos::TokenPos,
};

#[derive(Debug, PartialEq, PartialOrd)]
#[must_use]
pub(crate) struct TokenStream {
    token_kinds: Vec<TokenKind>,
    token_pos: Vec<TokenPos>,
    progress: usize,
}

impl TokenStream {
    pub(crate) fn new(
        token_kinds: Vec<TokenKind>,
        token_pos: Vec<TokenPos>,
    ) -> Self {
        Self {
            token_kinds,
            token_pos,
            progress: 0,
        }
    }
    pub(crate) fn toks(&self) -> &Vec<Token> {
        &self.token_kinds
    }

    pub(crate) fn kinds(&self) -> &TokenKind {
        &self.token_kinds[self.progress]
    }

    #[must_use]
    fn is_at_end(&self) -> bool {
        self.progress >= self.token_kinds.len()
    }
}
