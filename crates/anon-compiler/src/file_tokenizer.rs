use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use anon_core::interner::Interner;
use pest::iterators::{Pair, Pairs};

use crate::{line_tokenizer::*, token::Token, token_stream::TokenStream};

pub struct FileTokenizer<'a> {
    // 外部迭代器：提供 Rule::LINE 和 Rule::EOI
    outer_pairs: Pairs<'a, Rule>,

    // 内部迭代器：当前行的 Pairs，用于 LineTokenizer 或缩进计算
    current_line_pairs: Option<Pairs<'a, Rule>>,

    // LineTokenizer 实例：用于处理当前行的语句内容
    line_tokenizer: Option<LineTokenizer<'a>>,

    // 缩进状态
    indent_stack: Vec<usize>,

    // 输出缓冲区：用于存储 INDENT/DEDENT
    output_buffer: VecDeque<Token>,

    // Tab 宽度
    tab_width: usize,
    // ... 其他字段 ...
    interner: Rc<RefCell<Interner>>,
}

impl<'a> FileTokenizer<'a> {
    pub fn new(
        file_pair: Pair<'a, Rule>,
        tab_width: usize,
        interner: Rc<RefCell<Interner>>,
    ) -> Self {
        assert_eq!(file_pair.as_rule(), Rule::File);

        Self {
            outer_pairs: file_pair.into_inner(), // LINE, LINE, ..., EOI
            current_line_pairs: None,
            line_tokenizer: None,
            // 根缩进 [0]
            indent_stack: vec![0],
            output_buffer: VecDeque::new(),
            tab_width,
            // ... 初始化其他字段 ...
            interner,
        }
    }

    // 消耗并计算行首缩进，将剩余 Pairs 留在迭代器中
    fn calculate_indent(&self, line_pairs: &mut Pairs<'a, Rule>) -> usize {
        let mut current_indent: usize = 0;

        // 临时克隆迭代器，因为我们需要 peek 并可能消耗多个 Pair
        // 注意：pest::Pairs 实现了 Clone，这使得这种预读操作安全
        let mut temp_pairs = line_pairs.clone();

        loop {
            if let Some(pair) = temp_pairs.peek() {
                match pair.as_rule() {
                    Rule::SPACE => {
                        current_indent += pair.as_str().len();
                        temp_pairs.next(); // 消耗
                    }
                    Rule::TAB => {
                        current_indent += pair.as_str().len() * self.tab_width;
                        temp_pairs.next(); // 消耗
                    }
                    // 遇到非缩进 Token，停止计算
                    _ => break,
                }
            } else {
                // 行结束
                break;
            }
        }

        // 由于我们使用了 clone 进行预读，现在我们需要将原迭代器 line_pairs
        // 推进到相同的位置，以跳过已被计算的 SPACE/TAB Pairs。
        // 最简单的方法是直接跳过 line_pairs 中的 N 个元素
        let consumed_count = line_pairs.len() - temp_pairs.len();
        for _ in 0..consumed_count {
            line_pairs.next();
        }

        current_indent
    }

    fn inject_indent_tokens(&mut self, current_indent: usize) {
        let last_indent = *self.indent_stack.last().unwrap();

        if current_indent > last_indent {
            // INDENT
            self.indent_stack.push(current_indent);
            self.output_buffer.push_back(Token::Indent);
        } else if current_indent < last_indent {
            // DEDENT
            while self.indent_stack.last().unwrap_or(&0) > &current_indent {
                self.indent_stack.pop();
                self.output_buffer.push_back(Token::Dedent);
            }
            if *self.indent_stack.last().unwrap_or(&0) != current_indent {
                panic!("Inconsistent dedent level...");
            }
        }
        // 如果 current_indent == last_indent，则不做任何操作
    }

    fn handle_eoi(&mut self) {
        // EOI 时，输出所有未闭合的 DEDENT Tokens
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.output_buffer.push_back(Token::Dedent);
        }
    }
}

impl<'a> Iterator for FileTokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. 优先输出缓冲区中的 INDENT/DEDENT Tokens
        if let Some(token) = self.output_buffer.pop_front() {
            return Some(token);
        }

        // 2. 优先输出当前行 LineTokenizer 生成的 Tokens
        if let Some(ref mut lt) = self.line_tokenizer {
            if let Some(token) = lt.next() {
                // 如果是 NEWLINE Token，我们不应该在这里返回，而应该在处理完
                // 整行内容后，由 LineTokenizer 结束时返回。
                // 暂时假设 LineTokenizer 内部不返回 NEWLINE Token，而是
                // 让其消耗掉 NEWLINE Pair。
                return Some(token);
            } else {
                // LineTokenizer 已经结束（即当前行已处理完）
                self.line_tokenizer = None;

                // 确保在行内容结束后，我们输出一个 NEWLINE Token 作为语句分隔
                // ⚠️ 这取决于你的语言设计：是 NEWLINE 还是语句分隔符？
                // 假设你需要 NEWLINE token 来分隔语句
                return Some(Token::Newline);
            }
        }

        // 3. 当前行已处理完，从 outer_pairs 中获取下一行
        match self.outer_pairs.next() {
            Some(next_pair) => {
                match next_pair.as_rule() {
                    Rule::LINE => {
                        // 遇到新行，开始处理
                        let mut line_pairs = next_pair.into_inner();

                        // A. 缩进计算
                        let current_indent = self.calculate_indent(&mut line_pairs);

                        // B. 注入 INDENT/DEDENT
                        self.inject_indent_tokens(current_indent);

                        // C. 创建 LineTokenizer
                        // 将剩余的 line_pairs (ATOM* ~ _LINE_COMMENT? ~ NEWLINE) 传递给它
                        self.line_tokenizer = Some(LineTokenizer::new_line_pairs(
                            line_pairs,
                            self.tab_width,
                            self.interner.clone(), /* ... 其他参数 ... */
                        ));

                        // D. 递归调用 next() 来处理缓冲区或新的 LineTokenizer 输出
                        return self.next();
                    }
                    Rule::EOI => {
                        // 文件结束：输出所有 DEDENT Tokens
                        self.handle_eoi();
                        // 递归调用 next() 来输出剩余的 DEDENTs
                        return self.output_buffer.pop_front();
                    }
                    // 忽略其他顶层规则（如静默规则）
                    _ => self.next(),
                }
            }
            None => {
                // 文件结束
                return None;
            }
        }
    }
}

impl<'a> TokenStream<'a> for FileTokenizer<'a> {}

pub fn normalize_source_code_and_despace(raw_source: &str) -> String {
    let mut output = String::new();
    let in_string = false;
    let is_at_line_start = true;

    // 首先，确保文件以换行符结束（我们之前讨论过的规范化）
    let source_with_nl = if raw_source.ends_with('\n') {
        raw_source.to_string()
    } else {
        let mut s = raw_source.to_string();
        s.push('\n');
        s
    };

    for line in source_with_nl.lines() {
        let mut first_content_char_index = 0;
        let mut leading_whitespace = String::new();

        // 1. 识别并保留行首的缩进（SPACE/TAB）
        for (i, c) in line.chars().enumerate() {
            if c == ' ' || c == '\t' {
                leading_whitespace.push(c);
            } else {
                first_content_char_index = i;
                break;
            }
        }
        output.push_str(&leading_whitespace);

        // 2. 处理行内容：移除所有多余的空格，保留字面量内部的空格
        let content = &line[first_content_char_index..];
        let mut content_output = String::new();
        let mut in_quotes = false; // 简化处理，只检查单双引号

        for c in content.chars() {
            if c == '"' || c == '\'' {
                in_quotes = !in_quotes; // 切换引号状态
                content_output.push(c);
            } else if c.is_whitespace() && !in_quotes {
                // 如果在引号外部，忽略空格（或将其替换为单个占位符，如果你的ATOM*需要分隔符）
                // 鉴于你的 ATOM* 不允许分隔符，我们直接跳过。
                continue;
            } else {
                content_output.push(c);
            }
        }

        // 3. 将处理后的内容添加到输出
        output.push_str(&content_output);

        // 4. 确保行结束符（除了最后一行，我们之前已经加过了）
        // 这里必须使用原始的换行符，以防止 lines() 迭代器导致的丢失
        output.push('\n');
    }

    // 移除最后一次添加的额外换行符（lines() 迭代器的副作用）
    output.pop();

    output
}

pub fn normalize_ending_newline(raw_source: &str) -> String {
    // TODO: cannot handle \r\n
    if !raw_source.ends_with('\n') {
        format!("{}\n", raw_source)
    } else {
        raw_source.to_string()
    }
}

// 示例：
// 输入: "x = 1.1\n    y = 2"
// 输出: "x=1.1\n    y=2\n" (近似效果)

#[cfg(test)]
mod test {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_indent() {
        let test_str = "x=1\n    y=2\nz=3\n";
        let file = AnonParser::parse(Rule::File, test_str)
            .expect("successful parse")
            .next()
            .unwrap();

        let interner = Rc::new(RefCell::new(Interner::new()));
        let tokenizer = FileTokenizer::new(file, 4, interner.clone());

        let tokens: Vec<_> = tokenizer.collect();

        assert_eq!(tokens.len(), 14);
        assert_eq!(
            tokens.first(),
            Some(&Token::Identifier(interner.borrow_mut().intern_or_get("x")))
        );
        assert_eq!(
            tokens.get(1),
            Some(&Token::Keyword(interner.borrow_mut().intern_or_get("=")))
        );
        assert_eq!(
            tokens.get(2),
            Some(&Token::Literal(anon_ast::literal::Literal::Integer(1)))
        );
        assert_eq!(tokens.get(3), Some(&Token::Newline));
        // y=2
        assert_eq!(tokens.get(4), Some(&Token::Indent));
        assert_eq!(
            tokens.get(5),
            Some(&Token::Identifier(interner.borrow_mut().intern_or_get("y")))
        );
        assert_eq!(
            tokens.get(6),
            Some(&Token::Keyword(interner.borrow_mut().intern_or_get("=")))
        );
        assert_eq!(
            tokens.get(7),
            Some(&Token::Literal(anon_ast::literal::Literal::Integer(2)))
        );
        assert_eq!(tokens.get(8), Some(&Token::Newline));
        // z=3
        assert_eq!(tokens.get(9), Some(&Token::Dedent));
        assert_eq!(
            tokens.get(10),
            Some(&Token::Identifier(interner.borrow_mut().intern_or_get("z")))
        );
        assert_eq!(
            tokens.get(11),
            Some(&Token::Keyword(interner.borrow_mut().intern_or_get("=")))
        );
        assert_eq!(
            tokens.get(12),
            Some(&Token::Literal(anon_ast::literal::Literal::Integer(3)))
        );
        assert_eq!(tokens.get(13), Some(&Token::Newline));
    }

    #[test]
    fn test_normalize() {
        let raw_source = "x = 1.1\n    y = 2";
        let res = normalize_source_code_and_despace(raw_source);
        let expected = "x=1.1\n    y=2";
        assert_eq!(res, expected);

        let res = normalize_ending_newline(&res);
        let expected = "x=1.1\n    y=2\n";
        assert_eq!(res, expected);
    }
}
