pub(crate) struct SourceCode {
    code: String,
}

impl SourceCode {
    pub(crate) fn new(code: String) -> Self {
        Self { code }
    }
}
