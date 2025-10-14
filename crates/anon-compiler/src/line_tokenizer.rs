use std::{cell::RefCell, collections::VecDeque, rc::Rc, vec};

use anon_ast::literal::Literal;
use anon_core::interner::Interner;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

use crate::token::Token;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "anon.pest"]
pub struct AnonParser;

/// 请注意，LineTokenizer不负责输出NEWLINE，只会消耗。生成NEWLINE职责已经转移到了FileTokenizer
#[derive(Debug, Clone)]
pub struct LineTokenizer<'a> {
    // 原始 pest Pair 的迭代器
    inner: pest::iterators::Pairs<'a, Rule>,
    // 储存上一个读取的Pair，模拟实现peek效果
    peeked_pair: Option<Pair<'a, Rule>>,
    // 用于跟踪当前的缩进级别（栈）
    indent_stack: Vec<usize>,
    // 缓存 INDENT/DEDENT/Statement，以便按顺序输出
    output_buffer: VecDeque<Token>,
    // 记录是否在行首 (用于跳过空行)
    is_at_line_start: bool,
    // tab大小，用于兼容tab对应的空格数，通常为4
    tab_width: usize,
    interner: Rc<RefCell<Interner>>,
}

impl<'a> LineTokenizer<'a> {
    // par仅接受line层级rule LINE = { (SPACE | TAB)* ~ ATOM* ~ _LINE_COMMENT? ~ NEWLINE }
    pub fn new(pair: Pair<'a, Rule>, tab_width: usize, interner: Rc<RefCell<Interner>>) -> Self {
        assert_eq!(
            pair.as_rule(),
            Rule::LINE,
            "tokenizer should only receive line rule!"
        );
        // Inner pairs, including Space, TAB, ATOM, line comment and newline
        let inner_pairs = pair.into_inner();

        Self {
            inner: inner_pairs,
            peeked_pair: None,
            indent_stack: vec![],
            output_buffer: VecDeque::new(),
            is_at_line_start: true,
            tab_width,
            interner,
        }
    }

    pub fn new_line_pairs(
        pair: Pairs<'a, Rule>,
        tab_width: usize,
        interner: Rc<RefCell<Interner>>,
    ) -> Self {
        Self {
            inner: pair,
            peeked_pair: None,
            indent_stack: vec![],
            output_buffer: VecDeque::new(),
            is_at_line_start: true,
            tab_width: 4,
            interner,
        }
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.output_buffer.pop_front() {
            return Some(token);
        }
        let mut pair = self.peeked_pair.take().or_else(|| self.inner.next())?;
        loop {
            match pair.as_rule() {
                // A. 遇到换行符 (NEWLINE)
                Rule::NEWLINE => {
                    self.is_at_line_start = true;
                    return None;
                    return Some(Token::Newline);
                }

                Rule::ATOM => {
                    self.is_at_line_start = false;
                    let inner_pair = pair.into_inner().next().unwrap();
                    let token = match inner_pair.as_rule() {
                        Rule::KW_ANNOTATE
                        | Rule::KW_CASE
                        | Rule::KW_CLASS
                        | Rule::KW_ELSE
                        | Rule::KW_EXPORT
                        | Rule::KW_IF
                        | Rule::KW_IMPORT
                        | Rule::KW_IN
                        | Rule::KW_INSTANCE
                        | Rule::KW_LET
                        | Rule::KW_MATCH
                        | Rule::KW_EQ
                        | Rule::KW_THEN => Token::Keyword(
                            self.interner
                                .borrow_mut()
                                .intern_or_get(inner_pair.as_str()),
                        ),
                        Rule::FLOAT => {
                            Token::Literal(Literal::Float(inner_pair.as_str().parse().unwrap()))
                        }
                        Rule::INTEGER => {
                            Token::Literal(Literal::Integer(inner_pair.as_str().parse().unwrap()))
                        }

                        Rule::CHARACTER => {
                            let raw_char = inner_pair.as_str();
                            let content = &raw_char[1..raw_char.len() - 1];
                            Token::Literal(Literal::Char(content.parse().unwrap()))
                        }
                        Rule::STRING => {
                            // Remove surrounding quotes and intern the string value
                            let raw_str = inner_pair.as_str();
                            let content = raw_str[1..raw_str.len() - 1].to_string(); // naive unquote
                            Token::Literal(Literal::String(
                                self.interner.borrow_mut().intern_or_get(&content),
                            ))
                        }
                        Rule::IDENT => {
                            let ident = inner_pair.as_str();
                            Token::Identifier(self.interner.borrow_mut().intern_or_get(ident))
                        }

                        Rule::NEWLINE => {
                            return None;
                        }
                        // TODO!
                        _ => todo!(),
                    };
                    return Some(token);
                }

                _ => {}
            }

            if let Some(next_pair) = self.inner.next() {
                pair = next_pair;
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use super::*;
    #[test]
    fn test_int_assign() {
        let test_str1 = "x =1\n";
        let line = AnonParser::parse(Rule::LINE, test_str1)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let tokenizer = LineTokenizer::new(line, 4, interner.clone());

        let tokens: Vec<_> = tokenizer.collect();

        // 3 atoms
        assert_eq!(tokens.len(), 3);

        assert_eq!(
            tokens.first(),
            Some(&Token::Identifier(interner.borrow_mut().intern_or_get("x")))
        );

        assert_eq!(
            tokens.get(1),
            Some(&Token::Keyword(interner.borrow_mut().intern_or_get("=")))
        );

        assert_eq!(tokens.get(2), Some(&Token::Literal(Literal::Integer(1))));

        assert!(
            tokens.last().is_some() && (tokens.last() != Some(&Token::Newline)),
            "The end of tokens should not be NewLine!"
        );
    }

    #[test]
    fn test_negative_float_assign() {
        let test_str1 = "x=-1.1\n";
        let line = AnonParser::parse(Rule::LINE, test_str1)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let tokenizer = LineTokenizer::new(line, 4, interner.clone());

        let tokens: Vec<_> = tokenizer.collect();
        // 3 atoms
        assert_eq!(tokens.len(), 3);

        assert_eq!(
            tokens.first(),
            Some(&Token::Identifier(interner.borrow_mut().intern_or_get("x")))
        );

        assert_eq!(
            tokens.get(1),
            Some(&Token::Keyword(interner.borrow_mut().intern_or_get("=")))
        );

        assert_eq!(tokens.get(2), Some(&Token::Literal(Literal::Float(-1.1))));

        assert!(
            tokens.last().is_some() && (tokens.last() != Some(&Token::Newline)),
            "The end of tokens should not be NewLine!"
        );
    }

    #[test]
    fn test_line_comment() {
        let test_str1 = "-- This is a line comment!\n";
        let line = AnonParser::parse(Rule::LINE, test_str1)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let tokenizer = LineTokenizer::new(line, 4, interner.clone());

        let tokens: Vec<_> = tokenizer.collect();

        dbg!(&tokens);
        // 3 atoms
        assert_eq!(tokens.len(), 0);

        assert!(
            tokens.last() != Some(&Token::Newline),
            "The end of tokens should not be NewLine!"
        );
    }


}
