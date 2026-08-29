use crate::actions::lex;

use crate::types::{
    source_code::SourceCode, tokens::pos::TokenPos,
    tokens::token_stream::TokenStream,
};

use crate::types::tokens::kind::TokenKind;

#[test]
pub(crate) fn test_lexer() {
    let source_code = "var x";

    let source_code =
        SourceCode::new(source_code.to_string());

    let result = lex::lex(source_code);

    let tok_kinds = vec![
        TokenKind::Var,
        TokenKind::Ident("x".to_string()),
    ];

    match result {
        Ok(v) => assert_eq!(*v.kinds(), tok_kinds),
        Err(e) => panic!("{e:?}"),
    }
}
