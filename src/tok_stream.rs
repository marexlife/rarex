use crate::tok::Tok;

#[must_use]
pub(crate) struct TokStream {
    toks: Vec<Tok>,
    progress: usize,
}

impl TokStream {
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
