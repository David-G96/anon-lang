use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::intern::{Interner, Sym};
use crate::literal::Literal;
use pest::iterators::Pair;
use pest_derive::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "anon.pest"]
struct AnonParser;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Indent,
    Dedent,
    Newline,
    EOF,

    Identifier(Sym),
    Literal(Literal),
    Operator(Sym),
    Keyword(Sym),
    Delimiter(Sym),

    // 从 pest Pair 中提取的实际 Tokens
    #[deprecated]
    Statement(Sym),
    // 占位符，用于处理所有我们不关心的 pest Tokens
    Other(Rule),
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
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
    ///
    interner: Rc<RefCell<Interner>>,
}

impl<'a> Iterator for Tokenizer<'a> {
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
                    return Some(Token::Newline);
                }

                // B. 遇到缩进 (SPACE 或 TAB)
                Rule::SPACE | Rule::TAB if self.is_at_line_start => {
                    // --- 核心缩进计算逻辑 ---
                    let mut current_indent: usize = 0;

                    // 消耗所有连续的缩进 Tokens
                    let mut last_pair = pair; // 从当前 pair 开始计算

                    loop {
                        match last_pair.as_rule() {
                            Rule::SPACE => {
                                current_indent += last_pair.as_str().len();
                            }
                            Rule::TAB => {
                                // 将 Tab 转换为等效的空格数
                                current_indent +=
                                    last_pair.as_str().len() * self.tab_width;
                            }
                            // 遇到第一个非缩进 Token，停止计算并缓存它
                            _ => {
                                self.peeked_pair = Some(last_pair); // 缓存多读的 pair
                                break;
                            }
                        }

                        // 尝试消耗下一个 pair
                        if let Some(next_pair) = self.inner.next() {
                            last_pair = next_pair;
                        } else {
                            // 文件结束，并且最后的 tokens 都是缩进
                            break;
                        }
                    }
                    // --- 缩进计算和 INDENT/DEDENT 注入 ---

                    self.is_at_line_start = false;
                    let last_indent = *self.indent_stack.last().unwrap_or(&0);

                    if current_indent > last_indent {
                        // 缩进增加 (INDENT)
                        self.indent_stack.push(current_indent);
                        self.output_buffer.push_back(Token::Indent);
                    } else if current_indent < last_indent {
                        // 缩进减少 (DEDENT)
                        while self.indent_stack.last().unwrap_or(&0) > &current_indent {
                            self.indent_stack.pop();
                            self.output_buffer.push_back(Token::Dedent);
                        }
                        // 检查缩进是否对齐（Python 风格的严格缩进）
                        if *self.indent_stack.last().unwrap_or(&0) != current_indent {
                            panic!(
                                "Inconsistent dedent level: expected one of {:?}, got {}",
                                self.indent_stack, current_indent
                            );
                        }
                    }

                    // 再次检查缓冲区，然后继续 while 循环
                    if let Some(token) = self.output_buffer.pop_front() {
                        return Some(token);
                    }

                    // 如果缓冲区为空，且缓存了非缩进 Token，则继续外层循环
                    if self.peeked_pair.is_some() {
                        pair = self.peeked_pair.take().unwrap();
                        continue; // 继续处理缓存的非缩进 token
                    }
                }

                // D. 遇到 EOI
                Rule::EOI => {
                    // 文件结束时，输出所有未闭合的 DEDENT Tokens
                    while self.indent_stack.len() > 1 {
                        self.indent_stack.pop();
                        self.output_buffer.push_back(Token::Dedent);
                    }
                    // 再次检查缓冲区，输出最后的 DEDENTs
                    return self.output_buffer.pop_front();
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
                        | Rule::KW_THEN => Token::Keyword(
                            self.interner
                                .borrow_mut()
                                .intern_or_get(inner_pair.as_str()),
                        ),
                        // TODO!
                        _ => todo!(),
                    };
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
