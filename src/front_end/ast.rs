#[derive(Debug)]
pub struct AbstractSyntaxTree {
    pub expression: Expression,
}

#[derive(Debug)]
pub enum Expression {
    NumberLiteral(u32),
    BinaryOperator {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
}
