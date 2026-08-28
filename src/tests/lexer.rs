use crate::{
    actions::lexer::Lexer,
    types::{
        source_code::SourceCode,
        token::{Token, pos::TokenPos},
        token_stream::TokenStream,
    },
};

use crate::types::token::kind::TokenKind;

#[test]
pub(crate) fn test_lexer() {
    let source_code = "var x";

    let source_code =
        SourceCode::new(source_code.to_string());

    let result = Lexer::new().lex(source_code);

    let toks = vec![
        TokenKind::Var,
        TokenKind::Ident("x".to_string()),
    ];

    match result {
        Ok(v) => {
            for e in v.toks() {
                for f in &toks {
                    assert_eq!(e.kind(), f)
                }
            }
        }
        Err(e) => {
            assert!(false, "{e:?}");
        }
    }
}
