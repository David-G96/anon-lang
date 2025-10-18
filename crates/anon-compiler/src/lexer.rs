use core::panic;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use anon_core::interner::Interner;
use pest::iterators::{Pair, Pairs};

use crate::{
    line_tokenizer::{LineTokenizer, Rule},
    token::Token,
};

pub struct Lexer<'a> {
    // 外部迭代器：提供 Rule::LINE 和 Rule::EOI
    file_pairs: Pairs<'a, Rule>,
    // 缩进状态
    indent_stack: Vec<usize>,
    // 输出缓冲区：用于存储 INDENT/DEDENT
    output_buffer: VecDeque<Token>,
    // Tab 宽度
    tab_width: u32,
    // ... 其他字段 ...
    interner: Rc<RefCell<Interner>>,
}

impl<'a> Lexer<'a> {
    pub fn new(
        file_pair: Pair<'a, Rule>,
        tab_width: u32,
        interner: Rc<RefCell<Interner>>,
    ) -> Self {
        debug_assert_eq!(
            file_pair.as_rule(),
            Rule::File,
            "Lexer should only receive File rule"
        );

        Self {
            file_pairs: file_pair.into_inner(), // LINE, LINE, ..., EOI
            // 根缩进 [0]
            indent_stack: vec![0],
            output_buffer: VecDeque::new(),
            tab_width,
            interner,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 优先从缓存里读取
        if let Some(buffered_token) = self.output_buffer.pop_front() {
            return Some(buffered_token);
        }

        let file_pair = self.file_pairs.next()?;

        match file_pair.as_rule() {
            Rule::LINE => {
                let line_pairs = file_pair.into_inner();
                let line_tokenizer =
                    LineTokenizer::new_line_pairs(line_pairs, self.interner.clone());
                let (indent_count, tokens) = line_tokenizer.parse_line(self.tab_width);

                // 这一行不是空行，需要计算indent
                if tokens != vec![Token::Newline] {
                    if indent_count > *self.indent_stack.last().unwrap_or(&0) {
                        self.indent_stack.push(indent_count);
                        self.output_buffer.push_back(Token::Indent);
                    }
                    // Check if we need to decrease indentation
                    else if indent_count < *self.indent_stack.last().unwrap_or(&0) {
                        while indent_count < *self.indent_stack.last().unwrap_or(&0) {
                            self.indent_stack.pop();
                            self.output_buffer.push_back(Token::Dedent);
                        }
                        // After dedenting, the indentation should match exactly
                        if indent_count != *self.indent_stack.last().unwrap_or(&0) {
                            panic!("Inconsistent indentation!");
                        }
                    }
                    self.output_buffer.extend(tokens);
                } else {
                    self.output_buffer.extend(tokens);
                }
            }
            Rule::EOI => {
                while self.indent_stack.len() > 1 {
                    self.indent_stack.pop();
                    self.output_buffer.push_back(Token::Dedent);
                }

                self.output_buffer.push_back(Token::EOF);
            }
            x => {
                unreachable!("Unreachable file rule: {:#?}", x)
            }
        }
        self.output_buffer.pop_front()
    }
}

#[cfg(test)]
mod test {
    use pest::Parser;

    use crate::{keyword::Keyword, line_tokenizer::PestParser};

    use super::*;

    #[test]
    fn test_single_line() {
        let test_str = "x = 1 \n";
        let file = PestParser::parse(Rule::File, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let lexer = Lexer::new(file, 4, interner.clone());

        let tokens: Vec<_> = lexer.collect();

        let expected = vec![
            Token::Identifier(interner.borrow_mut().intern_or_get("x")),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(1)),
            Token::Newline,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_two_lines() {
        let test_str = "x=1\ny=2\n";
        let file = PestParser::parse(Rule::File, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let lexer = Lexer::new(file, 4, interner.clone());

        let tokens: Vec<_> = lexer.collect();

        let x_sym = interner.borrow_mut().intern_or_get("x");
        let y_sym = interner.borrow_mut().intern_or_get("y");

        let expected = vec![
            Token::Identifier(x_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(1)),
            Token::Newline,
            Token::Identifier(y_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(2)),
            Token::Newline,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_two_line_indent() {
        let test_str = "x=1\n    y=2\n";
        let file = PestParser::parse(Rule::File, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let lexer = Lexer::new(file, 4, interner.clone());

        let tokens: Vec<_> = lexer.collect();

        let x_sym = interner.borrow_mut().intern_or_get("x");
        let y_sym = interner.borrow_mut().intern_or_get("y");

        let expected = vec![
            Token::Identifier(x_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(1)),
            Token::Newline,
            Token::Indent,
            Token::Identifier(y_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(2)),
            Token::Newline,
            Token::Dedent,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_multiple_line_indent() {
        let test_str = "x=1\n    y=2\n    z = 3 \nphi = 4 \n";
        let file = PestParser::parse(Rule::File, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let lexer = Lexer::new(file, 4, interner.clone());

        let tokens: Vec<_> = lexer.collect();

        let x_sym = interner.borrow_mut().intern_or_get("x");
        let y_sym = interner.borrow_mut().intern_or_get("y");
        let z_sym = interner.borrow_mut().intern_or_get("z");
        let phi_sym = interner.borrow_mut().intern_or_get("phi");

        let expected = vec![
            Token::Identifier(x_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(1)),
            Token::Newline,
            Token::Indent,
            Token::Identifier(y_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(2)),
            Token::Newline,
            Token::Identifier(z_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(3)),
            Token::Newline,
            Token::Dedent,
            Token::Identifier(phi_sym),
            Token::Operator(crate::operator::Operator::Eq),
            Token::Literal(anon_ast::literal::Literal::Integer(4)),
            Token::Newline,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_if() {
        let test_str = "if x\n  then\n    y\n  else\n    z\n";
        let file = PestParser::parse(Rule::File, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let lexer = Lexer::new(file, 4, interner.clone());

        let tokens: Vec<_> = lexer.collect();

        let x_sym = interner.borrow_mut().intern_or_get("x");
        let y_sym = interner.borrow_mut().intern_or_get("y");
        let z_sym = interner.borrow_mut().intern_or_get("z");

        let expected = vec![
            Token::Keyword(Keyword::If),
            Token::Identifier(x_sym),
            Token::Newline,
            Token::Indent,
            Token::Keyword(Keyword::Then),
            Token::Newline,
            Token::Indent,
            Token::Identifier(y_sym),
            Token::Newline,
            Token::Dedent,
            Token::Keyword(Keyword::Else),
            Token::Newline,
            Token::Indent,
            Token::Identifier(z_sym),
            Token::Newline,
            Token::Dedent,
            Token::Dedent,
            Token::EOF,
        ];

        assert_eq!(expected, tokens);
    }
}
