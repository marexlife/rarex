use crate::types::token::{
    kind::TokenKind, pos::TokenPos,
};

#[derive(Debug, PartialEq, PartialOrd)]
#[must_use]
pub(crate) struct TokenStream {
    token_kinds: Vec<TokenKind>,
    token_poses: Vec<TokenPos>,
    progress: usize,
}

impl TokenStream {
    pub(crate) fn new(
        token_kinds: Vec<TokenKind>,
        token_poses: Vec<TokenPos>,
    ) -> Self {
        Self {
            token_kinds,
            token_poses,
            progress: 0,
        }
    }
    pub(crate) fn poses(&self) -> TokenPos {
        self.token_poses[self.progress]
    }

    pub(crate) fn kinds(&self) -> &TokenKind {
        &self.token_kinds[self.progress]
    }

    #[must_use]
    fn is_at_end(&self) -> bool {
        self.progress >= self.token_kinds.len()
    }
}
