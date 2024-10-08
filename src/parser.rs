use std::collections::VecDeque;

use crate::lexer::{Operator, Token};

/// Our expression can be defined recursively. 
/// Each expression is either a branching expression (+ 9 7) or a number (9). 
/// For example: (+ 9 0) => 
/// ```
/// Expr::(Value {
///     operation: Add, 
///     left: Expr::Number(9), 
///     right Expr::Number(0) 
/// })
/// ```
/// We see our Expr contains an Expr.
#[derive(Debug)]
pub enum Expr {
    // Recursive, as value contains 2 expressions, with Number case being the terminal base case
    // The Box<> can be ignored, it simply provides some help for the compiler.
    Value(Box<Value>),
    Number(f64),
}

#[derive(Debug)]
pub struct Value {
    pub operation: Operator,
    pub left_operand: Box<Expr>,
    pub right_operand: Box<Expr>,
}

/// Parses a token string into a valid expression
pub fn parse(token_string: &mut VecDeque<Token>) -> Expr {
    match token_string.pop_front().unwrap() {
        Token::Number(num) => Expr::Number(num),
        Token::Operator(operation) => {
            let left_operand = Box::new(parse(token_string));
            let right_operand = Box::new(parse(token_string));
            Expr::Value(Box::new(Value {
                operation,
                left_operand,
                right_operand
            }))
        }
    }
}

