use itertools::Itertools;
use std::collections::VecDeque;

use crate::lexer::{Operator, Token};

// A program is just a list of things we want to do really
pub type Program = Vec<Statement>;

// We can have print expressions or we can have a variable declaration
#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    PrintExpr(Expr),
}

// Because a variable declaration always has an identifier and an expression afterwards.
// We can't enforce an ident be valid at compile time through our types though (perhaps
// dependent typing could do that, idk enough about it)
#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub(crate) ident: char,
    pub(crate) expr: Expr,
}

// AST -> Abstract Syntax Tree (The tree made up of abstract types that represents our program)
// Define our AST => The tree is just prefix notation math, we all intrinsically use it without knowing
#[derive(Debug, Clone)]
pub enum Expr {
    Value(Box<VarOp>),
    Number(f64),
    Ident(char),
}

// Our operations can have 1 (unary) or 2 (binary) arguments, so we should separate them
// (this isn't necessarily the best way to do that, but it works)
#[derive(Debug, Clone)]
pub enum VarOp {
    BinOp(BinOpValue),
    UnOp(UnOpValue),
}

// Theoretically, at the parsing phase, in a more advanced parser
// we should use a separate AST BinOp type to differentiate BinOps and UnOps,
// but the SYA (Shunting yard algorithm) in this case ensures this for us,
// but it does cause some headaches should we want to optimize mega style
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct UnOpValue {
    pub operation: Operator,
    pub operand: Box<Expr>,
}

pub fn parse_program(token_string: &mut VecDeque<Token>) -> Program {
    let mut program = Vec::new();
    let mut statement_vec = Vec::new();
    let mut temp_vec = Vec::new();

    for token in token_string.drain(..) {
        match token {
            Token::SemiColon => {
                statement_vec.push(temp_vec.clone());
                temp_vec.clear();
            }
            token => temp_vec.push(token),
        }
    }

    for statement in statement_vec {
        // for fun, who needs if/else
        let line = match &statement.contains(&Token::Colon) {
            true => Statement::VariableDeclaration(parse_ident(statement)),
            false => Statement::PrintExpr(parse_expr(&mut statement.into())),
        };
        program.push(line);
    }

    program
}

fn parse_ident(token_string: Vec<Token>) -> VariableDeclaration {
    let mut iter = token_string.into_iter();
    let ident: char;

    if let Token::Ident(char) = iter.next().expect("Invalid syntax") {
        ident = char;
    } else {
        panic!("Invalid syntax");
    }

    if iter.next().expect("Invalid syntax") != Token::Colon {
        panic!("Invalid syntax");
    }

    let rest = iter.collect_vec();

    let expr = parse_expr(&mut rest.into());

    VariableDeclaration { ident, expr }
}

/// Parser our token string into an Expr
fn parse_expr(token_string: &mut VecDeque<Token>) -> Expr {
    // Depending on Number or Operator, either return a base case, or recursively call
    match token_string.pop_front().unwrap() {
        // Base case 1
        Token::Ident(ident) => Expr::Ident(ident),
        // Base case 2
        Token::Number(num) => Expr::Number(num),
        // Recursive Call for unary operations
        Token::Operator(Operator::Log) => {
            let operand = Box::new(parse_expr(token_string));
            Expr::Value(Box::new(VarOp::UnOp(UnOpValue {
                operation: Operator::Log,
                operand,
            })))
        }
        // Recursive Call for binary operations
        Token::Operator(operation) => {
            // Recursive Call for Left
            let left_operand = Box::new(parse_expr(token_string));
            // Recursive Call for Right
            let right_operand = Box::new(parse_expr(token_string));
            // Return our new value
            Expr::Value(Box::new(VarOp::BinOp(BinOpValue {
                operation,
                left_operand,
                right_operand,
            })))
        }
        _ => panic!("Error: Invalid Syntax"),
    }
}
