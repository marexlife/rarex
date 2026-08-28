#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum TokenKind {
    Var,
    Ident(String),
}

impl TokenKind {
    pub(crate) fn new(input: &str) -> Self {
        match input {
            "var" => TokenKind::Var,
            _ => TokenKind::Ident(input.to_string()),
        }
    }
}