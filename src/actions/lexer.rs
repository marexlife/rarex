use crate::types::source_code::SourceCode;
use crate::types::tok::{Tok, TokKind, TokPos};
use crate::types::token_stream::TokenStream;

#[must_use]
#[derive(Debug)]
pub(crate) enum LexerErr {
    SourceCodeEmpty,
}

#[derive(PartialEq, PartialOrd)]
enum LastCharKind {
    WasDefault,
    WasNotDefault,
}

#[must_use]
pub(crate) struct Lexer {
    last_word: String,
    toks: Vec<Tok>,
    pos: TokPos,
    last_char_kind: Option<LastCharKind>,
}

impl Lexer {
    pub(crate) fn new() -> Self {
        Self {
            last_word: String::new(),
            toks: vec![],
            pos: TokPos::new(),
            last_char_kind: None,
        }
    }

    fn is_flushable(&self) -> bool {
        match &self.last_char_kind {
            Some(v) => *v == LastCharKind::WasDefault,
            None => true,
        }
    }

    pub(crate) fn lex(
        mut self,
        source_code: SourceCode,
    ) -> Result<TokenStream, LexerErr> {
        if source_code.is_empty() {
            return Err(LexerErr::SourceCodeEmpty);
        }

        for source_code_char in source_code.chars() {
            self.last_char_kind =
                Some(LastCharKind::WasNotDefault);

            match source_code_char {
                ' ' | '\n' => self.flush_last(),
                ';' => self.flush_last_and_append(
                    source_code_char,
                ),
                _ => self.handle_default_case(
                    source_code_char,
                ),
            }
        }

        self.last_char_kind = None;

        Ok(TokenStream::new(self.toks))
    }

    fn handle_default_case(&mut self, c: char) {
        self.last_char_kind =
            Some(LastCharKind::WasDefault);

        self.last_word.push(c)
    }

    fn flush_last(&mut self) {
        self.toks.push(Tok::new(
            TokKind::new(&self.last_word.as_str()),
            self.pos,
        ));
        self.last_word.clear();
    }

    fn flush_last_and_append(
        &mut self,
        char_to_append: char,
    ) {
        if self.is_flushable() {
            self.flush_last();
        }

        let char_to_append_string =
            char_to_append.to_string();

        self.toks.push(Tok::new(
            TokKind::new(&char_to_append_string),
            self.pos,
        ));
    }
}
