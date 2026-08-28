use crate::types::source_code::SourceCode;
use crate::types::token::{
    kind::TokenKind, pos::TokenPos,
};
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
    token_kinds: Vec<TokenKind>,
    token_poses: Vec<TokenPos>,
    pos: TokenPos,
    last_char_kind: Option<LastCharKind>,
    progress: usize
}

impl Lexer {
    pub(crate) fn new() -> Self {
        Self {
            last_word: String::new(),
            token_kinds: vec![],
            token_poses: vec![],
            pos: TokenPos::new(),
            last_char_kind: None,
            progress: 0
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

        self.pos.advance_column();

        for source_code_char in source_code.chars() {
            self.last_char_kind =
                Some(LastCharKind::WasNotDefault);

            match source_code_char {
                '\n' => {
                    self.pos.advance_line();
                    self.flush_last();
                }
                ' ' => self.flush_last(),
                ';' => self.flush_last_and_append(
                    source_code_char,
                ),
                _ => self.handle_default_case(
                    source_code_char,
                ),
            }
        }

        Ok(TokenStream::new(
            self.token_kinds,
            self.token_poses,
        ))
    }

    fn handle_default_case(&mut self, c: char) {
        self.last_char_kind =
            Some(LastCharKind::WasDefault);

        self.last_word.push(c)
    }

    fn push(
        &mut self,
        token_kind: TokenKind
    ) {
        self.token_kinds.push(token_kind);
        self.token_poses.push(self.token_poses[self.progress]);
    }

    fn flush_last(token_kinds: Vec<TokenKind>) {
        self.push(
            token_kinds[self.progress],
        );
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

        self.push(
            TokenKind::new(&char_to_append_string),
            self.pos,
        );
    }
}
