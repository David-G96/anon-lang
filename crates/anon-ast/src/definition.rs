use crate::func_decl::FuncDecl;

/// Top level definition
#[non_exhaustive]
#[derive(Debug)]
pub enum Definition<M> {
    FuncDecl(FuncDecl<M>),
}
