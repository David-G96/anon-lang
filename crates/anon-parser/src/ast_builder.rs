use crate::{
    meta::NoMeta,
    tokenizer::{Token, Tokenizer},
    untyped_ast::Expr,
};

#[derive(Debug, Clone)]
pub struct AstBuilder<'a> {
    // 封装 IndentManager，它提供了我们的 Tokens 流
    tokens: Tokenizer<'a>,
    // 用于缓存 peek 过的 Tokens，因为递归下降需要前瞻
    // LanguageToken 已经是经过处理的 Tokens，所以现在我们用它来 peek
    peeked_token: Option<Token>,
}

impl<'a> AstBuilder<'a> {
    pub fn new(tokens: Tokenizer<'a>) -> Self {
        AstBuilder {
            tokens,
            peeked_token: None,
        }
    }

    // 获取下一个 Tokens，优先从缓存中获取
    fn next_token(&mut self) -> Option<Token> {
        self.peeked_token.take().or_else(|| self.tokens.next())
    }

    // 窥视下一个 Tokens，但不消耗
    fn peek(&mut self) -> Option<&Token> {
        if self.peeked_token.is_none() {
            self.peeked_token = self.tokens.next();
        }
        self.peeked_token.as_ref()
    }

    // 消耗当前 Tokens，并检查它是否符合期望
    fn consume(&mut self, expected: Token) -> Result<Token, String> {
        let token = self
            .next_token()
            .ok_or_else(|| format!("Expected {:?}, but reached end of file", expected))?;

        // 实际应用中，你可能只需要匹配 Token 的**类型**
        if token == expected {
            Ok(token)
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, token))
        }
    }

    pub fn parse(&mut self) -> Result<Expr<NoMeta>, String> {
        while let Some(tok) = self.next_token() {
        }

        unimplemented!()
    }
}
