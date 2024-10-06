use std::ops::{Add, Div, Mul, Sub};

use crate::parser::{BinOpValue, Expr, UnOpValue, VarOp};

pub fn eval_expr(expr: Expr) -> f64 {
    match expr {
        Expr::Number(num) => num,
        Expr::Value(value) => eval_value(*value),
    }
}

pub fn eval_value(val: VarOp) -> f64 {
    match val {
        VarOp::BinOp(binop) => eval_binop(binop),
        VarOp::UnOp(unop) => eval_unop(unop),
    }
}

pub fn eval_unop(unop: UnOpValue) -> f64 {
    [f64::ln].get(unop.operation as usize - 5)
        // Tokenizer operation type used for parser op type too, so 
        // we can't differentiate binop and unop operations on the 
        // operation level, instead we wrap it in an enum during parsing.
        // Parser still only identifies #/Ln as the only unop,
        // so the invariant is maintained, but not as an explicit
        // fact in the type system
        .unwrap()(
            eval_expr(*unop.operand)
        )
}

pub fn eval_binop(binop: BinOpValue) -> f64 {
    [f64::add, f64::sub, f64::mul, f64::div, f64::powf]
        .get(binop.operation as usize)
        .unwrap_or_else(|| unreachable!())(
        eval_expr(*binop.left_operand),
        eval_expr(*binop.right_operand),
    )
}
