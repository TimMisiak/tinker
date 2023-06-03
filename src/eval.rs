use crate::grammar::ast::{EvalExpr, ProgramExpr};

// TODO: Expression evaluation needs an evaluation context. Possibly including memory read, register read, and symbol names
pub fn evaluate_expression(expr: EvalExpr) -> u64 {
    match expr {
        EvalExpr::Number(x) => x,
        EvalExpr::Add(x, _, y) => evaluate_expression(*x) + evaluate_expression(*y),
    }
}

pub fn evaluate_program(expr: ProgramExpr) -> u64 {
    match expr {
        ProgramExpr::Expression(x) => evaluate_expression(*x)
    }
}