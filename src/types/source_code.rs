pub(crate) struct SourceCode {
    code: String,
}

impl SourceCode {
    pub(crate) fn new(code: String) -> Self {
        Self { code }
    }

    pub(crate) fn chars(&self) -> std::str::Chars<'_> {
        self.code.chars()
    }
}
