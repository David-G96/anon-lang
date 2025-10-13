use std::{cell::RefCell, collections::VecDeque, iter::Peekable, rc::Rc};

use anon_core::interner::Interner;
use pest::iterators::Pair;

use crate::{line_tokenizer::*, token::Token};

/// transform a File rule into tokens
pub struct FileTokenizer<'a> {
    iter: Peekable<pest::iterators::Pairs<'a, Rule>>,
    indent_stack: Vec<usize>,
    output_buffer: VecDeque<Token>,
    is_at_line_start: bool,
    tab_width: usize,
    interner: Rc<RefCell<Interner>>,
}

impl<'a> FileTokenizer<'a> {
    pub fn new(
        file_rule: Pair<'a, Rule>,
        tab_width: usize,
        interner: Rc<RefCell<Interner>>,
    ) -> Self {
        assert_eq!(file_rule.as_rule(), Rule::File, "rule should be file only");
        // Inner pairs, including Line and EOI
        let inner_pairs = file_rule.into_inner();

        Self {
            iter: inner_pairs.peekable(),
            indent_stack: vec![],
            output_buffer: VecDeque::new(),
            is_at_line_start: true,
            tab_width,
            interner,
        }
    }
}

impl<'a> Iterator for FileTokenizer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

/// 在解析之前，确保输入字符串以换行符结束
fn normalize_source_code(source: &str) -> String {
    if source.ends_with('\n') {
        source.to_string()
    } else {
        // 在末尾追加一个换行符
        let mut owned_string = source.to_string();
        owned_string.push('\n');
        owned_string
    }
}
