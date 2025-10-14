// token_stream.rs

// 假设你的 Token 类型定义在这里
use crate::token::Token;

/// 定义 TokenStream 接口
pub trait TokenStream<'a>: Iterator<Item = Token> {
    // 额外的方法，如果需要，可以用于调试或预读
    // 但核心是实现 Iterator<Item = Token>

    // 如果需要，可以添加一个返回行号/列号的方法
    // fn current_span(&self) -> Span;
}
