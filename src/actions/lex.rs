use crate::types::source_code::SourceCode;
use crate::types::token::kind::TokenConversionErrKind;
use crate::types::token::{
    kind::TokenKind, pos::TokenPos,
};
use crate::types::token_stream::TokenStream;

#[must_use]
#[derive(Debug)]
pub(crate) enum LexerErr {
    SourceCodeEmpty,
    ConversionErrKind(TokenConversionErrKind),
}

#[derive(PartialEq, PartialOrd)]
enum LastCharKind {
    WasDefault,
    WasNotDefault,
}

pub(crate) fn lex(
    source_code: SourceCode,
) -> Result<TokenStream, LexerErr> {
    let mut last_word: String = String::new();
    let mut token_kinds: Vec<TokenKind> = vec![];
    let mut token_poses: Vec<TokenPos> = vec![];
    let mut token_pos: TokenPos = TokenPos::new();
    let mut last_char_kind: Option<LastCharKind>;

    if source_code.is_empty() {
        return Err(LexerErr::SourceCodeEmpty);
    }

    token_pos.advance_column();

    for source_code_char in source_code.chars() {
        last_char_kind =
            Some(LastCharKind::WasNotDefault);

        match source_code_char {
            '\n' => {
                token_pos.advance_line();

                flush_last(
                    &mut token_kinds,
                    &mut token_poses,
                    &mut last_word,
                    token_pos,
                );
            }
            ' ' => flush_last(
                &mut token_kinds,
                &mut token_poses,
                &mut last_word,
                token_pos,
            ),
            ';' => {
                match flush_last_and_append(
                    &mut token_kinds,
                    &mut token_poses,
                    &mut last_word,
                    &last_char_kind,
                    source_code_char,
                    token_pos,
                ) {
                    Ok(_) => {}
                    Err(e) => return Err(
                        LexerErr::ConversionErrKind(e),
                    ),
                }
            }
            _ => handle_default_case(
                &mut last_char_kind,
                source_code_char,
                &mut last_word,
            ),
        }
    }

    flush_last(
        &mut token_kinds,
        &mut token_poses,
        &mut last_word,
        token_pos,
    );

    Ok(TokenStream::new(token_kinds, token_poses))
}

fn is_flushable(
    last_char_kind: &Option<LastCharKind>,
) -> bool {
    match last_char_kind {
        Some(v) => *v == LastCharKind::WasDefault,
        None => true,
    }
}

fn handle_default_case(
    last_char_kind: &mut Option<LastCharKind>,
    current_char: char,
    last_word: &mut String,
) {
    *last_char_kind = Some(LastCharKind::WasDefault);

    last_word.push(current_char)
}

fn push(
    token_kinds: &mut Vec<TokenKind>,
    token_poses: &mut Vec<TokenPos>,
    token_kind: TokenKind,
    token_pos: TokenPos,
) {
    token_kinds.push(token_kind);
    token_poses.push(token_pos);
}

fn flush_last(
    token_kinds: &mut Vec<TokenKind>,
    token_poses: &mut Vec<TokenPos>,
    last_word: &mut String,
    token_pos: TokenPos,
) {
    push(
        token_kinds,
        token_poses,
        TokenKind::from_str(last_word),
        token_pos,
    );

    last_word.clear();
}

fn flush_last_and_append(
    token_kinds: &mut Vec<TokenKind>,
    token_poses: &mut Vec<TokenPos>,
    last_word: &mut String,
    last_char_kind: &Option<LastCharKind>,
    char_to_append: char,
    token_pos: TokenPos,
) -> Result<(), TokenConversionErrKind> {
    if is_flushable(last_char_kind) {
        flush_last(
            token_kinds,
            token_poses,
            last_word,
            token_pos,
        );
    }

    let token_kind =
        match TokenKind::from_char(char_to_append) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

    push(
        token_kinds,
        token_poses,
        token_kind,
        token_pos,
    );

    Ok(())
}
