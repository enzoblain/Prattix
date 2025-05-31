use prattix::lexer::tokenize;

use std::io::{
    stdin, 
    stdout, 
    Write
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    // We prompt the user for an expression to calculate and store it in a String
    print!("Enter the expression to calculate: ");
    stdout().flush()?;
    stdin().read_line(&mut input)?;

    // Then we convert the inpute to a vector of tokens
    let vec = tokenize(input);

    println!("Digits: {:?}", vec);

    Ok(())
} 