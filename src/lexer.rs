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
    // We also check if 2 following charcters are not the same type
    let mut value = 1; // Assuming that an operator is not 0 and the expression starts with a number
    for c in s.chars() {
        if let Some(token) = get_token(c) {
            if value != token.value {
                value = token.value;
                vec.push(token);
            } else {
                println!("Excluding token: {:?}", token.item);
            }
        } else {
            println!("Skipping unrecognized character: {}", c);
        }
    }

    // If the last token is an operator, we remove it
    if value != 0 && !vec.is_empty() {
        vec.pop();
    }

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