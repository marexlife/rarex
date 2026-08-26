use crate::types::tok::Tok;

#[must_use]
pub(crate) struct TokenStream {
    toks: Vec<Tok>,
    progress: usize,
}

impl TokenStream {
    pub(crate) fn new(toks: Vec<Tok>) -> Self {
        Self { toks, progress: 0 }
    }

    pub(crate) fn at(&self) -> &Tok {
        &self.toks[self.progress]
    }

    #[must_use]
    fn is_at_end(&self) -> bool {
        self.progress >= self.toks.len()
    }
}
