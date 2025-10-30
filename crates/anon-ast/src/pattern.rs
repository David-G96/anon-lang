use crate::literal::Literal;

// 模式 (Pattern)
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub enum Pattern {
    // 匹配任何值，例如: `_`
    Wildcard,

    // 匹配一个字面量，例如: `5`, `true`
    Literal(Literal),

    // 将匹配到的值绑定到一个变量，例如: `x`
    Variable(String),

    // 构造器模式 (用于解构元组、列表或自定义代数数据类型)
    // - name: 构造器的名称，例如: 列表的 `Cons` 或元组的 `Tuple`
    // - args: 递归的子模式列表，例如: `(x, y)` 中的 `x` 和 `y`
    Constructor { name: String, args: Vec<Pattern> },

    // 模式别名：将匹配结果绑定到变量，同时进行解构，例如: `(x, y) @ point`
    Alias { name: String, pattern: Box<Pattern> },
}
