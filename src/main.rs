#![allow(dead_code)]

use eval::eval_program;
use lexer::lex;
use parser::parse_program;

pub mod lexer;
pub mod parser;
pub mod eval;
#[cfg(test)]
pub mod test;

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
