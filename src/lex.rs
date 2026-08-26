use crate::parse::TokStream;
use crate::tok::Tok;
use crate::tok::TokKind;
use crate::tok::TokPos;

#[must_use]
pub(crate) enum LexError {
    SourceCodeEmpty,
}

#[must_use]
pub(crate) struct SourceCode {
    source_code: String,
    last_word: String,
    tokens: TokStream,
    pos: TokPos,
}

impl SourceCode {
    pub(crate) fn new(source_code: String) -> Self {
        Self {
            source_code,
            last_word: String::new(),
            tokens: TokStream::new(),
            pos: TokPos::new(),
        }
    }

    pub(crate) fn lex(mut self) -> Result<TokStream, LexError> {
        for c in self.source_code.chars() {
            match c {
                ' ' | '\n' => {
                    let new_token_kind = match self.last_word.as_str() {
                        "var" => TokKind::Var,
                        ident => TokKind::Ident(ident.to_string()),
                    };

                    self.tokens.push(Tok::new(new_token_kind, self.pos));
                    self.last_word.clear();
                }
                ';' => {}
                _ => self.last_word.push(c),
            }
        }

        Ok(self.tokens)
    }
}
