// We store each token as Number or and Operator
// We just add an EndToken at the end of the vector
// to indicate the end of the input
#[derive(Clone, Debug)]
pub enum Item {
    Number(Number),
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

// The Number enum represents digits from 0 to 9
#[derive(Clone, Debug)]
pub enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

// This function serves us to know if it's a digit
// and to convert it to the corresponding Number enum variant
impl Number {
    pub fn from_digit(digit: u32) -> Option<Number> {
        match digit {
            0 => Some(Number::Zero),
            1 => Some(Number::One),
            2 => Some(Number::Two),
            3 => Some(Number::Three),
            4 => Some(Number::Four),
            5 => Some(Number::Five),
            6 => Some(Number::Six),
            7 => Some(Number::Seven),
            8 => Some(Number::Eight),
            9 => Some(Number::Nine),
            _ => None,
        }
    }

    pub fn get_value(number: Number) -> u32 {
        match number {
            Number::Zero => 0,
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        }
    }
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