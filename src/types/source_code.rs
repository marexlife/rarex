pub(crate) struct SourceCode<'a> {
    filepath: &'a str,
    code: String,
}

impl<'a> SourceCode<'a> {
    pub(crate) fn new(
        filepath: &'a str,
        code: String,
    ) -> Self {
        Self { filepath, code }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.code.is_empty()
    }

    pub(crate) fn filepath(&self) -> &str {
        &self.filepath
    }

    pub(crate) fn chars(&self) -> std::str::Chars<'_> {
        self.code.chars()
    }
}
