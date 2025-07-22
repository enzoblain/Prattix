// We store each token as Number or and Operator
// We just add an EndToken at the end of the vector
// to indicate the end of the input
#[derive(Clone, Debug)]
pub enum Item {
    Number(f64),
    Operator(Operator),
    EndToken,
}

// A token is defined by it's type (either a Number or an Operator)
// and a value that indicates its importance in the expression
#[derive(Clone, Debug)]
pub struct Token {
    pub item: Item,
    pub value: i32,
}

// The Operator enum represents the four basic arithmetic operations
#[derive(Clone, Debug)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// This function serves us to know if it's an operator
// and to convert it to the corresponding Operator enum variant
impl Operator {
    pub fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _ => None,
        }
    }
}