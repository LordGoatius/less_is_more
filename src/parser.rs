use std::collections::VecDeque;
use crate::lexer::{Operator, Token};

#[derive(Debug)]
pub enum Expr {
    Value(Box<Value>),
    Number(f64),
}

#[derive(Debug)]
pub struct Value {
    pub operation: Operator,
    pub left_operand: Box<Expr>,
    pub right_operand: Box<Expr>,
}

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

