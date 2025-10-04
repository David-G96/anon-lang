#![allow(dead_code)]

use crate::types::Type;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct NoMeta;

#[derive(Debug, PartialEq, Clone)]
pub struct TypeMeta {
    pub node_type: Type, // 这里的 Type 就是我们之前定义的 Type 枚举
                         // ... 未来你可能还会添加位置信息 (Span), 唯一 ID 等
}
