#![allow(dead_code)]

use eval::eval_expr;
use lexer::lex;
use parser::parse;
pub mod lexer;
pub mod parser;
pub mod eval;
#[cfg(test)]
pub mod test;

fn main() {
    let input = "+ 9 - 8 * 4 / 6 ^ 9 7".to_string();
    let token_string = lex(input);
    let ast = parse(&mut token_string.into());
    let value = eval_expr(ast);
    println!("{value}");
}
