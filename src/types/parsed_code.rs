pub(crate) struct ParsedCode {
    pub(crate) file_node: FileNode,
}

#[derive(Debug)]
pub(crate) enum TypeKind {
    Float(f32),
    Int(i32),
}

#[derive(Debug)]
pub(crate) struct VarDeclNode {
    pub(crate) var_name: String,
    pub(crate) var_type: TypeKind,
}

#[derive(Debug)]
pub(crate) enum FuncItemKind {
    VarDecl(VarDeclNode),
}

#[derive(Debug)]
pub(crate) struct FunNode {
    pub(crate) return_type: Option<TypeKind>,
    pub(crate) args_types: Vec<TypeKind>,
    pub(crate) func_body: Vec<FuncItemKind>,
}

#[derive(Debug)]
pub(crate) enum ItemKind {
    FuncItem(FunNode),
}

#[must_use]
#[derive(Debug)]
pub(crate) struct FileNode {
    pub(crate) items: Vec<ItemKind>,
}
