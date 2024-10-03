#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    // +
    Add,
    // -
    Sub,
    // *
    Mul,
    // /
    Div,
    // ^
    Pow,
}

pub fn lex(input: String) -> Vec<Token> {
    // all tokens are split by one or more spaces
    let split_by_space = input.split_whitespace();
    let mut token_string: Vec<Token> = Vec::new();

    for item in split_by_space {
        if item.len() == 1 {
            match item.chars().next().unwrap() {
                '+' => token_string.push(Token::Operator(Operator::Add)),
                '-' => token_string.push(Token::Operator(Operator::Sub)),
                '*' => token_string.push(Token::Operator(Operator::Mul)),
                '/' => token_string.push(Token::Operator(Operator::Div)),
                '^' => token_string.push(Token::Operator(Operator::Pow)),
                //
                num @ '0'..='9' => {
                    let digit = num.to_digit(10).unwrap();
                    token_string.push(Token::Number(digit.into()))
                }
                extra => println!("Lexer Error: {extra}"),
            }
        } else {
            let num: f64 = item.parse().expect("Invalid Program");
            token_string.push(Token::Number(num));
        }
    }

    return token_string;
}
