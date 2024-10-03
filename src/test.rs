
#[cfg(test)]
pub mod test {
    use std::collections::VecDeque;

    use crate::{lexer::{lex, Operator, Token}, parser::parse};

    #[test]
    fn text_lexer() {
        let input = "+ 9 - 8 * 4 / 6 ^ 9 7";
        let token_string = lex(input.to_owned());

        let test_tkn_str = vec![
            Token::Operator(Operator::Add),
            Token::Number(9.0),
            Token::Operator(Operator::Sub),
            Token::Number(8.0),
            Token::Operator(Operator::Mul),
            Token::Number(4.0),
            Token::Operator(Operator::Div),
            Token::Number(6.0),
            Token::Operator(Operator::Pow),
            Token::Number(9.0),
            Token::Number(7.0),
        ];

        assert_eq!(test_tkn_str, token_string);
        println!("{input:?}");
        println!("{token_string:?}");
    }

    #[test]
    fn test_parsing() {
        let input = "+ 9 - 8 * 4 / 6 ^ 9 7".to_string();
        let mut token_string: VecDeque<Token> = lex(input).into();
        let ast = parse(&mut token_string);
        println!("Ast: {ast:#?}");
    }
}
