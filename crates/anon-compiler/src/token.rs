use anon_ast::literal::Literal;

use crate::{delimiter::Delimiter, keyword::Keyword, lexer::Rule, operator::Operator};

pub type Sym = anon_core::interner::Symbol;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Token {
    Indent,
    Dedent,
    Newline,
    EOF,

    Identifier(Sym),
    Literal(Literal),
    Operator(Operator),
    Keyword(Keyword),
    Delimiter(Delimiter),

    // 从 pest Pair 中提取的实际 Tokens
    #[deprecated]
    Statement(Sym),
    // 占位符，用于处理所有我们不关心的 pest Tokens
    Other(Rule),
}
