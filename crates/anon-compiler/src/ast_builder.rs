use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

use anon_ast::func_decl::FuncDecl;
use anon_core::interner::Interner;

use crate::{
    Lexer, token::Token, untyped_ast::UntypedAST,
};

// flow： 优先从buffer的front读取，否则就从
#[derive(Debug, Clone)]
pub struct Parser<'a> {
    // 封装 IndentManager，它提供了我们的 Tokens 流
    lexer: Lexer<'a>,
    // 用于缓存 peek 过的 Tokens，因为递归下降需要前瞻
    // LanguageToken 已经是经过处理的 Tokens，所以现在我们用它来 peek
    buffered_tokens: VecDeque<Token>,
    interner: Rc<RefCell<Interner>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>, interner: Rc<RefCell<Interner>>) -> Self {
        Parser {
            lexer,
            buffered_tokens: VecDeque::new(),
            interner,
        }
    }

    // 获取下一个 Tokens，优先从缓存中获取
    fn next_token(&mut self) -> Option<Token> {
        self.buffered_tokens.pop_front().or(self.lexer.next())
    }

    // 窥视下一个 Tokens，但不消耗
    fn peek(&mut self) -> Option<&Token> {
        if self.buffered_tokens.is_empty() {
            if let Some(tok) = self.lexer.next() {
                self.buffered_tokens.push_back(tok);
            }
        }
        self.buffered_tokens.front()
    }

    /// 把元素放回缓存的头
    fn put_back(&mut self, tok: Token) {
        self.buffered_tokens.push_front(tok);
    }

    // 消耗当前 Tokens，并检查它是否符合期望
    fn consume(&mut self, expected: Token) -> Result<Token, String> {
        let token = self.next_token().ok_or_else(|| {
            format!("Expected {:?}, but reached end of file", expected)
        })?;

        // 实际应用中，你可能只需要匹配 Token 的**类型**
        if token == expected {
            Ok(token)
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, token))
        }
    }

    fn parse_func_deck(&mut self) -> Result<FuncDecl<()>, String> {
        if let Some(Token::Identifier(func_name)) = self.peek() {
            // get the params
            let mut params = vec![];
            while let Some(tok) = self.next_token() {
                if let Token::Identifier(param) = tok {
                    params.push(param);
                } else {
                    self.buffered_tokens.push_front(tok);
                    break;
                }
            }
            // get the func body

            unimplemented!()
        } else {
            Err("func_decl should begin with an identifier")?
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = UntypedAST;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.peek() {
            // TODO!
            todo!();
        } else {
            return None;
        }

        unimplemented!()
    }
}
