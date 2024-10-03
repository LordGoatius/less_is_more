use crate::{lexer::Operator, parser::{Expr, Value}};

pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Number(num) => num,
        Expr::Value(value) => eval_value(*value),
    }
}

pub fn eval_value(val: Value) -> f64 {
    match val.operation {
        Operator::Add => eval_expr(*val.left_operand) + eval_expr(*val.right_operand),
        Operator::Mul => eval_expr(*val.left_operand) * eval_expr(*val.right_operand),
        Operator::Sub => eval_expr(*val.left_operand) - eval_expr(*val.right_operand),
        Operator::Div => eval_expr(*val.left_operand) / eval_expr(*val.right_operand),
        Operator::Pow => eval_expr(*val.left_operand).powf(eval_expr(*val.right_operand)),
    }
}
