use crate::{lexer::Operator, parser::Expr};

/// We want to figure out what our expression actually is. To do this, we recursively traverse our
/// AST. When we get to a number, we return it, and we eventually get to a case where we meet
/// an operation where we only get 2 numbers. There we can evaluate that experession, returning
/// the result, and cascading through our tree until we arrive to our final value.
pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Number(num) => num,
        Expr::Value {
            operation,
            left_operand,
            right_operand
        } => {
            match operation {
                Operator::Add => eval_expr(*left_operand) + eval_expr(*right_operand),
                Operator::Mul => eval_expr(*left_operand) * eval_expr(*right_operand),
                Operator::Sub => eval_expr(*left_operand) - eval_expr(*right_operand),
                Operator::Div => eval_expr(*left_operand) / eval_expr(*right_operand),
                Operator::Pow => eval_expr(*left_operand).powf(eval_expr(*right_operand)),
            }
        }
    }
}
