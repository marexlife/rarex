use crate::tok::Tok;

#[must_use]
pub(crate) struct TokStream {
    toks: Vec<Tok>,
    progress: usize,
}

impl TokStream {
    pub(crate) fn new() -> Self {
        Self {
            toks: vec![],
            progress: 0,
        }
    }

    pub(crate) fn push(&mut self, tok: Tok) {
        self.toks.push(tok);
    }
}
