use crate::types::source_code::SourceCode;
use crate::types::tok::{Tok, TokKind, TokPos};
use crate::types::token_stream::TokenStream;

#[must_use]
#[derive(Debug)]
pub(crate) enum LexerErr {
    SourceCodeEmpty,
}

#[must_use]
pub(crate) struct Lexer {
    last_word: String,
    toks: Vec<Tok>,
    pos: TokPos,
}

impl Lexer {
    pub(crate) fn new() -> Self {
        Self {
            last_word: String::new(),
            toks: vec![],
            pos: TokPos::new(),
        }
    }

    pub(crate) fn lex(mut self, source_code: SourceCode) -> Result<TokenStream, LexerErr> {
        for c in source_code.chars() {
            match c {
                ' ' | '\n' => self.flush(),
                ';' => {}
                _ => self.last_word.push(c),
            }
        }

        Ok(TokenStream::new(self.toks))
    }

    fn flush(&mut self) {
        let new_token_kind = match self.last_word.as_str() {
            "var" => TokKind::Var,
            ident => TokKind::Ident(ident.to_string()),
        };

        self.toks.push(Tok::new(new_token_kind, self.pos));
        self.last_word.clear();
    }
}
