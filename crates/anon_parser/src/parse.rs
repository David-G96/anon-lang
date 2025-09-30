use std::fs::File;

use pest::Parser;
use pest_derive::Parser;

#[allow(dead_code)]
#[derive(Parser)]
#[grammar = "anon.pest"]
struct AnonParser;

use pest::iterators::Pair;
use std::collections::VecDeque;

// 假设这是你的 MyParser 结构体，以及生成的 Rule 枚举
// #[derive(Parser)]
// #[grammar = "anon.pest"]
// struct MyParser;
// #[derive(Debug, Clone, PartialEq)]
// enum Rule { /* ... 所有 pest 规则 ... */ }

// 1. 定义最终的 Tokens 集合，包含 INDENT 和 DEDENT
#[derive(Debug, Clone)]
pub enum LanguageToken {
    // 虚拟 Tokens
    Indent,
    Dedent,
    // 从 pest Pair 中提取的实际 Tokens
    Statement(String),
    Newline,
    // 占位符，用于处理所有我们不关心的 pest Tokens
    Other(Rule),
}

// 2. 缩进管理器结构
pub struct IndentManager<'a> {
    // 原始 pest Pair 的迭代器
    inner: pest::iterators::Pairs<'a, Rule>,
    // 储存上一个读取的Pair，模拟实现peek效果
    peeked_pair: Option<Pair<'a, Rule>>,
    // 用于跟踪当前的缩进级别（栈）
    indent_stack: Vec<usize>,
    // 缓存 INDENT/DEDENT/Statement，以便按顺序输出
    output_buffer: VecDeque<LanguageToken>,
    // 记录是否在行首 (用于跳过空行)
    is_at_line_start: bool,
    // tab大小，用于兼容tab对应的空格数，通常为4
    tab_width: usize,
}
// ... (IndentManager 结构体定义保持不变)

impl<'a> Iterator for IndentManager<'a> {
    type Item = LanguageToken;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. 优先输出缓冲区中的虚拟 Tokens (INDENT/DEDENT)
        if let Some(token) = self.output_buffer.pop_front() {
            return Some(token);
        }

        // 2. 获取下一个 Pair：优先从缓存中获取，其次从迭代器中获取
        let mut pair = self.peeked_pair.take().or_else(|| self.inner.next())?;

        // 3. 循环处理 Tokens
        loop {
            match pair.as_rule() {
                // A. 遇到换行符 (NEWLINE)
                Rule::NEWLINE => {
                    self.is_at_line_start = true;
                    return Some(LanguageToken::Newline);
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
                                current_indent += last_pair.as_str().len() * self.tab_width;
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
                        self.output_buffer.push_back(LanguageToken::Indent);
                    } else if current_indent < last_indent {
                        // 缩进减少 (DEDENT)
                        while self.indent_stack.last().unwrap_or(&0) > &current_indent {
                            self.indent_stack.pop();
                            self.output_buffer.push_back(LanguageToken::Dedent);
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

                // C. 遇到语句 (STATEMENT)
                Rule::STATEMENT => {
                    self.is_at_line_start = false;
                    return Some(LanguageToken::Statement(pair.as_str().to_string()));
                }

                // D. 遇到 EOI
                Rule::EOI => {
                    // 文件结束时，输出所有未闭合的 DEDENT Tokens
                    while self.indent_stack.len() > 1 {
                        self.indent_stack.pop();
                        self.output_buffer.push_back(LanguageToken::Dedent);
                    }
                    // 再次检查缓冲区，输出最后的 DEDENTs
                    return self.output_buffer.pop_front();
                }

                // E. 忽略其他 Tokens (行内空格、注释等)
                _ => {
                    self.is_at_line_start = false;
                }
            }

            // 如果执行到这里，说明 pair 已经被处理，获取下一个 pair
            if let Some(next_pair) = self.inner.next() {
                pair = next_pair;
            } else {
                return None; // 迭代器耗尽
            }
        }
    }
}

pub fn parse(s: &str) {
    let file = AnonParser::parse(Rule::file, s)
        .expect("unsuccessful parse")
        .next()
        .unwrap();
    for record in file.into_inner() {
        match record.as_rule() {
            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_anon() {}

    #[derive(Parser)]
    #[grammar = "csv.pest"] // relative to src
    struct CSVParser;

    #[test]
    fn test_csv() {
        let unparsed_file = std::fs::read_to_string("src/numbers.csv").expect("cannot read file");
        let file = CSVParser::parse(Rule::file, &unparsed_file)
            .expect("unsuccessful parse") // unwrap the parse result
            .next()
            .unwrap(); // get and unwrap the `file` rule; never fails

        let mut field_sum: f64 = 0.0;
        let mut record_count: u64 = 0;

        for record in file.into_inner() {
            match record.as_rule() {
                Rule::record => {
                    record_count += 1;

                    for field in record.into_inner() {
                        field_sum += field.as_str().parse::<f64>().unwrap();
                    }
                }
                Rule::NEWLINE => (), // ✅ 新增：忽略 NEWLINE 规则
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }

        println!("Sum of fields: {}", field_sum);
        println!("Number of records: {}", record_count);
    }
}
