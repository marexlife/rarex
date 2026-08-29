#[must_use]
#[derive(Debug)]
pub(crate) enum TokenConversionErrKind {
    UnknownChar,
}

#[must_use]
#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum TokenKind {
    Var,
    StatementEnd,
    Colon,
    Ident(String),
}

impl TokenKind {
    pub(crate) fn from_str(input: &str) -> Self {
        match input {
            "var" => TokenKind::Var,
            _ => TokenKind::Ident(input.to_string()),
        }
    }

    pub(crate) fn from_string(input: String) -> Self {
        match input.as_str() {
            "var" => TokenKind::Var,
            _ => TokenKind::Ident(input.to_string()),
        }
    }

    pub(crate) fn from_char(
        input: char,
    ) -> Result<Self, TokenConversionErrKind> {
        match input {
            ';' => Ok(TokenKind::StatementEnd),
            ':' => Ok(TokenKind::Colon),
            _ => Err(
                TokenConversionErrKind::UnknownChar,
            ),
        }
    }
}
