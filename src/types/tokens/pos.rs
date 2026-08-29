#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd,
)]
pub(crate) struct TokenPos {
    line: usize,
    column: usize,
}

impl TokenPos {
    pub(crate) fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub(crate) fn advance_column(&mut self) {
        self.column += 1;
    }

    pub(crate) fn advance_line(&mut self) {
        self.line += 1;
        self.column = 0;
    }
}

impl std::fmt::Display for TokenPos {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let display_name =
            format!("{}:{}", self.line, self.column);

        write!(f, "{}", display_name)
    }
}
