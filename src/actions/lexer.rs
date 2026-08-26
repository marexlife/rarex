use crate::source_code::SourceCode;
use crate::tok::Tok;
use crate::tok::TokKind;
use crate::tok::TokPos;
use crate::tok_stream::TokenStream;

#[must_use]
#[derive(Debug)]
pub(crate) enum LexerErr {
    SourceCodeEmpty,
}

#[must_use]
pub(crate) struct Lexer {
    source_code: SourceCode,
    last_word: String,
    toks: Vec<Tok>,
    pos: TokPos,
}

impl Lexer {
    pub(crate) fn new(source_code: SourceCode) -> Self {
        Self {
            source_code,
            last_word: String::new(),
            toks: vec![],
            pos: TokPos::new(),
        }
    }

    pub(crate) fn lex(mut self) -> Result<TokenStream, LexerErr> {
        for c in self.source_code.chars() {
            match c {
                ' ' | '\n' => {
                    let new_token_kind = match self.last_word.as_str() {
                        "var" => TokKind::Var,
                        ident => TokKind::Ident(ident.to_string()),
                    };

                    self.toks.push(Tok::new(new_token_kind, self.pos));
                    self.last_word.clear();
                }
                ';' => {}
                _ => self.last_word.push(c),
            }
        }

        Ok(TokenStream::new(self.toks))
    }
}
