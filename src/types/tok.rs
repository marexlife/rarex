pub(crate) enum TokKind {
    Var,
    Ident(String),
}

impl TokKind {
    pub(crate) fn new(input: &str) -> Self {
        match input {
            "var" => TokKind::Var,
            _ => TokKind::Ident(input.to_string()),
        }
    }
}

impl std::fmt::Display for TokKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = match *self {
            Self::Var => "variable",
            _ => "identifier",
        };

        write!(f, "{}", display_name)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TokPos {
    line: usize,
    column: usize,
}

impl TokPos {
    pub(crate) fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub(crate) fn advance(&mut self) {
        self.column += 1;
    }

    pub(crate) fn line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

impl std::fmt::Display for TokPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_name = format!("{}:{}", self.line, self.column);

        write!(f, "{}", display_name)
    }
}

pub(crate) struct Tok {
    kind: TokKind,
    pos: TokPos,
}

impl Tok {
    pub(crate) fn new(kind: TokKind, pos: TokPos) -> Self {
        Self { kind, pos }
    }
}

impl std::fmt::Display for Tok {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "on {},\n{}", self.pos, self.kind)
    }
}
