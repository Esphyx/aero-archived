pub struct AbstractSyntaxTree {
    expression: Expr,
}

pub enum Expr {
    BinaryOperator {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
}

pub enum BinaryOperator {
    Plus,
    Minus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Variable(String),
    Arrow {
        input: Box<Expression>,
        output: Box<Expression>,
    },
    Lambda {
        parameter: String,
        of_type: Box<Expression>,
        body: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        argument: Box<Expression>,
    },
}
