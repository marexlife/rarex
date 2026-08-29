#[derive(Debug)]
pub(crate) enum TypeKind {
    Float(f32),
    Int(i32),
}

#[derive(Debug)]
pub(crate) struct VarDecl {
    var_name: String,
    var_type: TypeKind,
}

#[derive(Debug)]
pub(crate) enum FuncItemKind {
    VarDecl(VarDecl),
}

#[derive(Debug)]
pub(crate) struct Func {
    return_type: TypeKind,
    args_types: Vec<TypeKind>,
    func_body: Vec<FuncItemKind>,
}

#[derive(Debug)]
pub(crate) enum ItemKind {
    FuncItem(Func),
}

#[must_use]
#[derive(Debug)]
pub(crate) struct ParsedCode {
    items: Vec<ItemKind>,
}

impl ParsedCode {
    pub(crate) fn new() -> Self {
        Self { items: vec![] }
    }
}
