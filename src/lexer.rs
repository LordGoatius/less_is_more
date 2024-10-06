#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Operator {
    // +
    Add = 0,
    // -
    Sub = 1,
    // *
    Mul = 2,
    // /
    Div = 3,
    // ^
    Pow = 4,
    // #,
    Log = 5,
}

/// Lex a valid input string into a vector of Tokens
pub fn lex(input: String) -> Vec<Token> {
    // all tokens are split by one or more spaces
    let split_by_space = input.split_whitespace(); // Equivalent to `input.split()` in python
    let mut token_string: Vec<Token> = Vec::new(); // Equivalent to `token_string = []` in python

    for item in split_by_space { // Maps pretty cleanly to python for loops
        // Check if it might be an operator
        if item.len() == 1 {
            match item.chars().next().unwrap() {
                // Check if it's an operator
                '+' => token_string.push(Token::Operator(Operator::Add)),
                '-' => token_string.push(Token::Operator(Operator::Sub)),
                '*' => token_string.push(Token::Operator(Operator::Mul)),
                '/' => token_string.push(Token::Operator(Operator::Div)),
                '^' => token_string.push(Token::Operator(Operator::Pow)),
                '#' => token_string.push(Token::Operator(Operator::Log)),
                // check if a single digit
                num @ '0'..='9' => {
                    let digit = num.to_digit(10).unwrap();
                    token_string.push(Token::Number(digit.into()))
                }
                // Anything else (invalid, we can choose to error or ignore here)
                extra => println!("Lexer Error: {extra}"),
            }
        } else {
            // otherwise, we assume it's a number, because that's the only valid
            // multiple character syntax
            let num: f64 = item.parse().expect("Invalid Program");
            token_string.push(Token::Number(num));
        }
    }

    return token_string;
}
