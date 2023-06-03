use crate::ast::*;

pub fn evaluate_expression(expr: Expression) -> u64 {
    match expr {
        Expression::Number(x) => x,
        Expression::Add(x, y) => evaluate_expression(*x) + evaluate_expression(*y),
        Expression::Subtract(x, y) => evaluate_expression(*x) - evaluate_expression(*y),
        Expression::Multiply(x, y) => evaluate_expression(*x) * evaluate_expression(*y),
        Expression::Divide(x, y) => evaluate_expression(*x) / evaluate_expression(*y),
    }
}

pub fn evaluate_program(expr: Program) -> u64 {
    match expr {
        Program::Exp(x) => evaluate_expression(*x)
    }
}
