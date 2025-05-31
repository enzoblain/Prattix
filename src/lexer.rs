use crate::token::{
    Item, 
    Number, 
    Operator, 
    Token
};

// Returns all the tokens in a string
pub fn tokenize(s: String) -> Vec<Token> {
    let mut vec = Vec::new();
    
    // Iterate through each character in the string
    // and convert it to a token if it's a digit or an operator
    // If it's not a digit or an operator, we skip it
    for c in s.chars() {
        if let Some(token) = get_token(c) {
            vec.push(token);
        }
    }

    // Finally, we add an end token to the vector
    vec.push(Token {
        item: Item::EndToken,
        value: -1,

    });

    vec
}

// Return a token if the character is a digit or an operator
pub fn get_token(c: char) -> Option<Token> {
    if let Some(digit) = Number::from_digit(c.to_digit(10).unwrap_or(10)) {
        Some(
            Token {
                item: Item::Number(digit),
                value: 0,
            }
        )
    } else if let Some(op) = Operator::from_char(c) {
        Some(Token {
            item: Item::Operator(op.clone()),
            // Set operator precedence based on the operator type
            value: match op {
                Operator::Add => 2,
                Operator::Subtract => 2,
                Operator::Multiply => 3,
                Operator::Divide => 3,
            },
        })
    } else {
        // We don't care about whitespace
        // we juste notify the user about unrecognized characters
        if !c.is_whitespace() {
            println!("Skipping unrecognized character: {}", c);
        }
        None
    }
}