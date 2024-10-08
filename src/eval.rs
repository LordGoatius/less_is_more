use crate::{lexer::Operator, parser::{Expr, Value}};

/// We want to figure out what our expression actually is. To do this, we recursively traverse our
/// AST. When we get to a number, we return it, and we eventually get to a case where we meet
/// an operation where we only get 2 numbers. There we can evaluate that experession, returning
/// the result, and cascading through our tree until we arrive to our final value.
pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Number(num) => num,
        Expr::Value(value) => eval_value(*value),
    }
}

fn eval_value(val: Value) -> f64 {
    match val.operation {
        Operator::Add => eval_expr(*val.left_operand) + eval_expr(*val.right_operand),
        Operator::Mul => eval_expr(*val.left_operand) * eval_expr(*val.right_operand),
        Operator::Sub => eval_expr(*val.left_operand) - eval_expr(*val.right_operand),
        Operator::Div => eval_expr(*val.left_operand) / eval_expr(*val.right_operand),
        Operator::Pow => eval_expr(*val.left_operand).powf(eval_expr(*val.right_operand)),
    }
}
