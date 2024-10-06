use std::collections::VecDeque;
use crate::lexer::{Operator, Token};

// Define our AST => The tree is just math, we all intrinsically use it without knowing
#[derive(Debug)]
pub enum Expr {
    Value(Box<VarOp>),
    Number(f64),
}

#[derive(Debug)]
pub enum VarOp {
    BinOp(BinOpValue),
    UnOp(UnOpValue)
}
#[derive(Debug)]
pub struct BinOpValue {
    pub operation: Operator,
    pub left_operand: Box<Expr>,
    pub right_operand: Box<Expr>,
}

#[derive(Debug)]
pub struct UnOpValue {
    pub operation: Operator,
    pub operand: Box<Expr>,
}

pub fn parse(token_string: &mut VecDeque<Token>) -> Expr {
    // Depending on Number or Operator, either return a base case, or recursively call
    match token_string.pop_front().unwrap() {
        // Base Base
        Token::Number(num) => Expr::Number(num),
        // Recursive Call
        Token::Operator(Operator::Log) => {
            let operand = Box::new(parse(token_string));
            Expr::Value(Box::new(VarOp::UnOp(UnOpValue {
                operation: Operator::Log,
                operand,
            })))
        }
        Token::Operator(operation) => {
            // Recursive Call for Left
            let left_operand = Box::new(parse(token_string));
            // Recursive Call for Right
            let right_operand = Box::new(parse(token_string));
            // Return our new value
            Expr::Value(Box::new(VarOp::BinOp(BinOpValue {
                operation,
                left_operand,
                right_operand
            })))
        }
    }
}

