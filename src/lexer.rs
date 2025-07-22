use crate::token::{
    Item,  
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
    if c.is_ascii_digit() {
        // If the character is a digit, we return a Number token
        Some(Token {
            item: Item::Number(c.to_digit(10).unwrap() as f64),
            value: 0, // Assuming all numbers have the same precedence
        })
    } else if let Some(op) = Operator::from_char(c) {
        Some(Token {
            item: Item::Operator(op.clone()),
            // Set operator precedence based on the operator type
            value: match op {
                Operator::Add => 1,
                Operator::Subtract => 1,
                Operator::Multiply => 2,
                Operator::Divide => 2,
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