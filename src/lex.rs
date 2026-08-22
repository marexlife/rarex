pub(crate) enum Tok {
    Var,
    Ident(String),
}
pub(crate) struct Lexer {
    source_code: String,
    last_word: String,
    tokens: Vec<Tok>,
}

impl Lexer {
    pub(crate) fn new(source_code: String) -> Self {
        Self {
            source_code,
            last_word: String::new(),
            tokens: vec![],
        }
    }

    pub(crate) fn lex(mut self) -> Vec<Tok> {
        for c in self.source_code.chars() {
            match c {
                ' ' => {
                    let new_token = match self.last_word.as_str() {
                        "var" => Tok::Var,
                        ident => Tok::Ident(ident.to_string()),
                    };

                    self.tokens.push(new_token);
                    self.last_word.clear();
                }
                _ => {
                    self.last_word.push(c);
                }
            }
        }

        self.tokens
    }
}
