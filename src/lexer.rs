use crate::token::{
    Item,  
    Operator, 
    Token
};

pub fn tokenize(s: String) -> Vec<Token> {
    let mut vec = Vec::new();
    let mut num = None;

    for n in s.chars() {
        // If the character is a digit, we append it to the current number
        if n.is_ascii_digit() {
            if let Some(ref mut number) = num {
                *number = format!("{}{}", number, n);
            } else {
                num = Some(n.to_string());
            }
        } else if let Some(op) = Operator::from_char(n) {
            // If the character is an operator, we first check if we have a number to add
            // If we do, we add it to the vector
            if let Some(number) = num.take() {
                vec.push(Token {
                    item: Item::Number(number.parse().unwrap()),
                    value: 0, // Assuming all numbers have the same precedence
                });
            }

            // Then we add the operator to the vector
            vec.push(Operator::get_token(op));
            
        // We skip non-digit and non-operator characters
        } else if !n.is_whitespace() {
            println!("Skipping unrecognized character: {}", n);
        }
    }    

    // If we have a number left at the end, we add it to the vector
    if let Some(number) = num {
        vec.push(Token {
            item: Item::Number(number.parse().unwrap()),
            value: 0, // Assuming all numbers have the same precedence
        });
    } 

    // If the last token is an operator, we delete it
    if let Some(Token { item: Item::Operator(_), .. }) = vec.last() {
        vec.pop();
    }

    vec
}