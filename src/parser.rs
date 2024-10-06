use std::collections::VecDeque;
use crate::lexer::{Operator, Token};

// AST -> Abstract Syntax Tree (The tree made up of abstract types that represents our program)
// Define our AST => The tree is just prefix notation math, we all intrinsically use it without knowing
#[derive(Debug)]
pub enum Expr {
    Value(Box<VarOp>),
    Number(f64),
}

// Our operations can have 1 (unary) or 2 (binary) arguments, so we should separate them
// (this isn't necessarily the best way to do that, but it works)
#[derive(Debug)]
pub enum VarOp {
    BinOp(BinOpValue),
    UnOp(UnOpValue)
}

// Theoretically, at the parsing phase, in a more advanced parser
// we should use a separate AST BinOp type to differentiate BinOps and UnOps,
// but the SYA (Shunting yard algorithm) in this case ensures this for us,
// but it does cause some headaches should we want to optimize mega style
#[derive(Debug)]
pub struct BinOpValue {
    pub operation: Operator,
    pub left_operand: Box<Expr>,
    pub right_operand: Box<Expr>,
}

/// We can see the problem here:
/// Our UnOpValue type allows for any crate::lexer::Operator to be the operation, which is an invalid
/// operation. Our type system allows us to represent an invalid state, which is usually a
/// big no no when using Abstract Data Types. Nevertheless, invariants are maintained in
/// the parser which ensure this doesn't happen, but nothing stops
/// ```rust
/// let invalid = UnOpValue {
///     operation: Operator::Add,
///     operand: Box::new<Expr::Number(6)>
/// }
/// ```
/// from being constructed manually.
/// Obviously, we cannot add 6 and nothing, but this is not maintained in our type system.
/// This is a warning to not mix Parser and Lexer types. I do it all the time, do as I say,
/// not as I do.
#[derive(Debug)]
pub struct UnOpValue {
    pub operation: Operator,
    pub operand: Box<Expr>,
}

/// Parser our token string into an Expr
pub fn parse(token_string: &mut VecDeque<Token>) -> Expr {
    // Depending on Number or Operator, either return a base case, or recursively call
    match token_string.pop_front().unwrap() {
        // Base Base
        Token::Number(num) => Expr::Number(num),
        // Recursive Call for unary operations
        Token::Operator(Operator::Log) => {
            let operand = Box::new(parse(token_string));
            Expr::Value(Box::new(VarOp::UnOp(UnOpValue {
                operation: Operator::Log,
                operand,
            })))
        }
        // Recursive Call for binary operations
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

