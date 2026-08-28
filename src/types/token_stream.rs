use crate::types::token::Token;

#[derive(Debug, PartialEq, PartialOrd)]
#[must_use]
pub(crate) struct TokenStream {
    toks: Vec<Token>,
    progress: usize,
}

impl TokenStream {
    pub(crate) fn new(toks: Vec<Token>) -> Self {
        Self { toks, progress: 0 }
    }
    pub(crate) fn toks(&self) -> &Vec<Token> {
        &self.toks
    }

    pub(crate) fn at(&self) -> &Token {
        &self.toks[self.progress]
    }

    #[must_use]
    fn is_at_end(&self) -> bool {
        self.progress >= self.toks.len()
    }
}
