use pest_derive::Parser;
use std::{cell::RefCell, rc::Rc};

use crate::{keyword::Keyword::*, operator::Operator};
use anon_ast::literal::Literal;
use anon_core::interner::Interner;
use pest::iterators::{Pair, Pairs};

use crate::token::Token;

#[derive(Parser)]
#[grammar = "anon.pest"]
pub struct PestParser;

/// 单行lexer
#[derive(Debug, Clone)]
pub struct LineTokenizer<'a> {
    // 原始 pest Pair 的迭代器
    pairs: pest::iterators::Pairs<'a, Rule>,
    // String Interner
    interner: Rc<RefCell<Interner>>,
    buffer: Option<Pair<'a, Rule>>,
}

impl<'a> LineTokenizer<'a> {
    /// pair仅接受line层级rule LINE = { (SPACE | TAB)* ~ ATOM* ~ _LINE_COMMENT? ~ NEWLINE }
    pub fn new(pair: Pair<'a, Rule>, interner: Rc<RefCell<Interner>>) -> Self {
        assert_eq!(
            pair.as_rule(),
            Rule::LINE,
            "tokenizer should only receive line rule!"
        );
        // Inner pairs, including Space, TAB, ATOM, line comment and newline
        let inner_pairs = pair.into_inner();

        Self {
            pairs: inner_pairs,
            interner,
            buffer: None,
        }
    }
    ///  你需要自己确保Pairs都是来自line
    pub fn new_line_pairs(
        pairs: Pairs<'a, Rule>,
        interner: Rc<RefCell<Interner>>,
    ) -> Self {
        Self {
            pairs,
            interner,
            buffer: None,
        }
    }

    /// parse the whole line, returns the indent count and the rest tokens,
    pub fn parse_line(mut self, tab_width: u32) -> (usize, Vec<Token>) {
        let mut indent_count = 0usize;

        loop {
            match self.pairs.next() {
                Some(pair) => match pair.as_rule() {
                    Rule::INDENT => {
                        let indent_text = pair.as_str();
                        indent_count += indent_text.chars().fold(0usize, |acc, c| {
                            if c == '\t' {
                                acc + tab_width as usize
                            } else {
                                acc + 1
                            }
                        });
                    }
                    _ => {
                        self.buffer = Some(pair);
                        break;
                    }
                },
                None => break,
            }
        }

        // collect remaining tokens from the same iterator we used above
        (indent_count, self.collect())
    }
}

impl<'a> Iterator for LineTokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 优先从缓存里读取，然后从迭代器self.pairs里读取，最后再返回None
        let line_pair = self.buffer.take().or_else(|| self.pairs.next())?;
        match line_pair.as_rule() {
            Rule::NEWLINE => Some(Token::Newline),
            Rule::ATOM => {
                let atom_pair = line_pair.into_inner().next().unwrap();
                let token = match atom_pair.as_rule() {
                    Rule::KW_CASE => Token::Keyword(Case),
                    Rule::KW_CLASS => Token::Keyword(Class),
                    Rule::KW_DATA => Token::Keyword(Data),
                    Rule::KW_ELSE => Token::Keyword(Else),
                    Rule::KW_EXPORT => Token::Keyword(Export),
                    Rule::KW_IF => Token::Keyword(If),
                    Rule::KW_IMPORT => Token::Keyword(Import),
                    Rule::KW_IN => Token::Keyword(In),
                    Rule::KW_INSTANCE => Token::Keyword(Instance),
                    Rule::KW_LET => Token::Keyword(Let),
                    Rule::KW_MATCH => Token::Keyword(Match),
                    Rule::KW_THEN => Token::Keyword(Then),
                    Rule::KW_TYPE => Token::Keyword(Type),
                    Rule::FLOAT => Token::Literal(Literal::Float(
                        atom_pair.as_str().parse().unwrap(),
                    )),
                    Rule::INTEGER => Token::Literal(Literal::Integer(
                        atom_pair.as_str().parse().unwrap(),
                    )),
                    Rule::CHARACTER => {
                        let raw_char = atom_pair.as_str();
                        let content = &raw_char[1..raw_char.len() - 1];
                        Token::Literal(Literal::Char(content.parse().unwrap()))
                    }
                    Rule::STRING => {
                        // Remove surrounding quotes and intern the string value
                        let raw_str = atom_pair.as_str();
                        let content = raw_str[1..raw_str.len() - 1].to_string(); // naive unquote
                        Token::Literal(Literal::String(
                            self.interner.borrow_mut().intern_or_get(&content),
                        ))
                    }
                    Rule::IDENT => {
                        let ident = atom_pair.as_str();
                        Token::Identifier(
                            self.interner.borrow_mut().intern_or_get(ident),
                        )
                    }
                    Rule::NEWLINE => {
                        return None;
                    }
                    Rule::OP_ADD => Token::Operator(Operator::Add),
                    Rule::OP_NEG => Token::Operator(Operator::Negate),
                    Rule::OP_EQ => Token::Operator(Operator::Eq),
                    Rule::OP_MUL => Token::Operator(Operator::Mul),
                    Rule::DELIMITER_ANNOTATE => {
                        Token::Delimiter(crate::delimiter::Delimiter::Annotate)
                    }
                    Rule::DELIMITER_COMMA => {
                        Token::Delimiter(crate::delimiter::Delimiter::Comma)
                    }
                    Rule::DELIMITER_LPAREN => {
                        Token::Delimiter(crate::delimiter::Delimiter::LParen)
                    }
                    Rule::DELIMITER_RPAREN => {
                        Token::Delimiter(crate::delimiter::Delimiter::RParen)
                    }
                    Rule::DELIMITER_UNDERSCORE => {
                        Token::Delimiter(crate::delimiter::Delimiter::UnderScore)
                    }

                    y => {
                        unreachable!("Unreachable Atom rule: {:#?}", y)
                    }
                };
                Some(token)
            }
            Rule::SPACE => self.next(),

            x => {
                unreachable!("Unreachable Line rule: {:#?}", x)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use pest::Parser;

    use super::*;

    use Literal::*;
    use Operator::*;

    #[test]
    fn test_empty() {
        let test_str = "\n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<Token> = line_tokenizer.collect();
        let expected: Vec<Token> = vec![Token::Newline];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_int_assign() {
        let test_str = "foo =1 \n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<_> = line_tokenizer.collect();
        let expected: Vec<_> = vec![
            Token::Identifier(interner.borrow_mut().intern_or_get("foo")),
            Token::Operator(Operator::Eq),
            Token::Literal(Literal::Integer(1)),
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_negative_float_assign() {
        let test_str = "bar = -1.1 \n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<_> = line_tokenizer.collect();
        let expected: Vec<_> = vec![
            Token::Identifier(interner.borrow_mut().intern_or_get("bar")),
            Token::Operator(Eq),
            Token::Operator(Negate),
            Token::Literal(Float(1.1)),
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_line_comment() {
        let test_str = "--345\n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<_> = line_tokenizer.collect();
        let expected: Vec<_> = vec![Token::Newline];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_character_assign() {
        let test_str = "c = 'c' \n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<_> = line_tokenizer.collect();
        let expected: Vec<_> = vec![
            Token::Identifier(interner.borrow_mut().intern_or_get("c")),
            Token::Operator(Eq),
            Token::Literal(Char('c')),
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_string_assign() {
        let test_str = "s = \"I love the way you lie.\" \n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let tokens: Vec<Token> = line_tokenizer.collect();
        let s_sym = { interner.borrow_mut().intern_or_get("s") };
        let lit_sym = {
            interner
                .borrow_mut()
                .intern_or_get("I love the way you lie.")
        };

        let expected = vec![
            Token::Identifier(s_sym),
            Token::Operator(Eq),
            Token::Literal(Literal::String(lit_sym)), // 假设 Token::Literal 接受 Symbol
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_parse_line_space() {
        let test_str = "    x = 1 \n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let (indent_count, tokens) = line_tokenizer.parse_line(4);

        assert_eq!(4, indent_count);

        let s_sym = { interner.borrow_mut().intern_or_get("x") };

        let expected = vec![
            Token::Identifier(s_sym),
            Token::Operator(Eq),
            Token::Literal(Literal::Integer(1)), // 假设 Token::Literal 接受 Symbol
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_parse_line_tab() {
        let test_str = "\ts = \"I love the way you lie.\"\n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let (indent_count, tokens) = line_tokenizer.parse_line(4);

        assert_eq!(4, indent_count);

        let s_sym = { interner.borrow_mut().intern_or_get("s") };
        let lit_sym = {
            interner
                .borrow_mut()
                .intern_or_get("I love the way you lie.")
        };

        let expected = vec![
            Token::Identifier(s_sym),
            Token::Operator(Eq),
            Token::Literal(Literal::String(lit_sym)), // 假设 Token::Literal 接受 Symbol
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_parse_line_hybrid() {
        let test_str = "  \t      s = \"I love the way you lie.\"\n";
        let line = PestParser::parse(super::Rule::LINE, test_str)
            .expect("unsuccessful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let line_tokenizer = LineTokenizer::new(line, interner.clone());

        let (indent_count, tokens) = line_tokenizer.parse_line(4);

        assert_eq!(12, indent_count);

        let s_sym = { interner.borrow_mut().intern_or_get("s") };
        let lit_sym = {
            interner
                .borrow_mut()
                .intern_or_get("I love the way you lie.")
        };

        let expected = vec![
            Token::Identifier(s_sym),
            Token::Operator(Eq),
            Token::Literal(Literal::String(lit_sym)), // 假设 Token::Literal 接受 Symbol
            Token::Newline,
        ];

        assert_eq!(expected, tokens);
    }
}
