#![allow(dead_code)]

/// 1\. This is our module for taking text and turning it into chunks (our token string) that the parser can understand.
pub mod lexer;
/// 2\. This is our module for taking a token string and turning it into an AST (abstract syntax
/// tree) that the evaluator can understand.
pub mod parser;
/// 3\. This is our module for taking our AST and executing it.
pub mod eval;
#[cfg(test)]
pub mod test;
//pub mod compiler;

use lexer::lex;
use parser::parse_program;
use eval::eval_program;

fn main() {
    let input = 
    r#"
    + 9 - 8 * 4 / 6 ^ 9 7 ;
    a : + 6 7 ;
    + 0 a ;
    "#.into();
    let token_string = lex(input);
    let ast = parse_program(&mut token_string.into());
    eval_program(ast);
}
