use crate::grammar::parse_tree::*;

#[derive(Debug)]
pub enum Program
{
    Exp(Box<Expression>)
}

#[derive(Debug)]
pub enum Expression
{
    Number(u64),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

fn ast_from_basicexpr(expr: BasicExpr) -> Box<Expression> {
    match expr {
        BasicExpr::Number(x) => Box::new(Expression::Number(x)),
        BasicExpr::Parens(_, x, _) => ast_from_addexpr(*x),
    }
}

fn ast_from_multexpr(expr: MultExpr) -> Box<Expression> {
    match expr {
        MultExpr::Multiply(x, _, y) => Box::new(Expression::Multiply(ast_from_basicexpr(*x), ast_from_multexpr(*y))),
        MultExpr::Divide(x, _, y) => Box::new(Expression::Divide(ast_from_basicexpr(*x), ast_from_multexpr(*y))),
        MultExpr::BasicExp(x) => ast_from_basicexpr(*x),
    }
}

fn ast_from_addexpr(expr: AddExpr) -> Box<Expression> {
    match expr {
        AddExpr::Add(x, _, y) => Box::new(Expression::Add(ast_from_multexpr(*x), ast_from_addexpr(*y))),
        AddExpr::Subtract(x, _, y) => Box::new(Expression::Subtract(ast_from_multexpr(*x), ast_from_addexpr(*y))),
        AddExpr::MultExp(x) => ast_from_multexpr(*x),
    }
}

pub fn ast_from_parse_tree(expr: ProgramExpr) -> Program {
    match expr {
        ProgramExpr::Expression(x) => Program::Exp(ast_from_addexpr(*x))
    }
}
