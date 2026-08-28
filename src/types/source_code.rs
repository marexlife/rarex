pub(crate) struct SourceCode {
    code: String,
}

impl SourceCode {
    pub(crate) fn new(code: String) -> Self {
        Self { code }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub(crate) fn chars(&self) -> std::str::Chars<'_> {
        self.code.chars()
    }
}
