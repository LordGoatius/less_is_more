use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

use crate::parser::{BinOpValue, Expr, Program, Statement, UnOpValue, VarOp, VariableDeclaration};

/// Public function to evalue a program with state (variable declaration)
pub fn eval_program(ast: Program) {
    let mut vars: HashMap<char, Expr> = HashMap::new();

    for statement in ast {
        match statement {
            Statement::PrintExpr(expr) => {
                let value = eval_expr(expr, &vars);
                println!("{value}");
            }
            Statement::VariableDeclaration(VariableDeclaration { ident, expr }) => {
                vars.insert(ident, expr);
            }
        }
    }
}

/// Evaluate an expression into a f64. Any valid Expr which comes from the `crate::parser::parse`
/// function should be valid, and cannot fail.
fn eval_expr(expr: Expr, vars: &HashMap<char, Expr>) -> f64 {
    match expr {
        Expr::Number(num) => num,
        Expr::Value(value) => eval_value(*value, vars),
        Expr::Ident(ident) => {
            let value = vars
                .get(&ident)
                .expect(&format!("Variable {ident} not defined")[..]);
            eval_expr(value.clone(), vars)
        }
    }
}

/// Evaluate a value into an f64
fn eval_value(val: VarOp, vars: &HashMap<char, Expr>) -> f64 {
    match val {
        VarOp::BinOp(binop) => eval_binop(binop, vars),
        VarOp::UnOp(unop) => eval_unop(unop, vars),
    }
}

/// Evaluate a value into an f64
fn eval_unop(unop: UnOpValue, vars: &HashMap<char, Expr>) -> f64 {
    [f64::ln]
        .get(unop.operation as usize - 5)
        // Tokenizer operation type used for parser op type too, so
        // we can't differentiate binop and unop operations on the
        // operation level, instead we wrap it in an enum during parsing.
        // Parser still only identifies #/Ln as the only unop,
        // so the invariant is maintained, but not as an explicit
        // fact in the type system.
        .unwrap()(eval_expr(*unop.operand, vars))
}

/// We should be doing some crazy optimizations here, if the compiler is smart enough.
/// We represent the operation enum as a u8. We can use this fact to convert it into a
/// `usize` which can index a dispatch/function table. This conversion cannot fail, but
/// I did not explicitly denote this in code. The compiler can choose to optimize or not
/// optimize this. However, I added an unreachable! in the `get().unwrap_or_else()` call, signifying
/// that I believe the unwrap cannot fail, and thus the binop has to be between `0..5`.
/// This is the main reason to have BinOp and UnOp be different operations in our parser.
/// Because `binop.operation = 5` is possible in our type system, we cannot be guaranteed
/// this conversion cannot index the array at an invalid location.
fn eval_binop(binop: BinOpValue, vars: &HashMap<char, Expr>) -> f64 {
    [f64::add, f64::sub, f64::mul, f64::div, f64::powf]
        .get(binop.operation as usize)
        .unwrap_or_else(|| unreachable!())(
        eval_expr(*binop.left_operand, vars),
        eval_expr(*binop.right_operand, vars),
    )
}
