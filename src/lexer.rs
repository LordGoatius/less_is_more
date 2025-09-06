/// All possible tokens that we care about, including the wrappers for ones that represent numbers
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Ident(char),
    Colon,
    SemiColon,
}

/// All operators that we can represent using a single symbol
#[derive(Debug, PartialEq, Clone, Copy)]
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
        // Check if it might be an operator, because all operators are length 1
        if item.len() == 1 {
            // Match on the 1 character we grabbed
            match item.chars().next().unwrap() {
                // Check if it's an operator
                '+' => token_string.push(Token::Operator(Operator::Add)),
                '-' => token_string.push(Token::Operator(Operator::Sub)),
                '*' => token_string.push(Token::Operator(Operator::Mul)),
                '/' => token_string.push(Token::Operator(Operator::Div)),
                '^' => token_string.push(Token::Operator(Operator::Pow)),
                '#' => token_string.push(Token::Operator(Operator::Log)),
                ';' => token_string.push(Token::SemiColon),
                ':' => token_string.push(Token::Colon),
                // check if a single digit
                num @ '0'..='9' => {
                    let digit = num.to_digit(10).unwrap();
                    token_string.push(Token::Number(digit.into()))
                }
                // Anything else (we assume it is an ident)
                extra => token_string.push(Token::Ident(extra))
            }
        } else {
            // otherwise, we assume it's a number, because that's the only valid
            // multiple character syntax
            let num: f64 = item.parse().expect(&format!("Invalid Program: {item}"));
            token_string.push(Token::Number(num));
        }
    }

    return token_string;
}
