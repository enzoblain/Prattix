use crate::{Item, Number, Operator, Token};

pub fn evaluate(mut expression: Vec<Token>) -> u32 {
    while expression.len() > 1 {
        let mut place = 0;
        let mut value = expression[1].value;

        let mut first_number = expression[0].item.clone();
        let mut operator = expression[1].item.clone();
        let mut second_number = expression[2].item.clone();

        for i in 3..expression.len() {
            let item = &expression[i];

            if item.value == 0 {
                continue;
            }

            if item.value > value {
                place = i - 1;
                value = item.value;
                first_number = expression[i - 1].item.clone();
                operator = item.item.clone();
                second_number = expression[i + 1].item.clone();
            }
        }

        let ans = calculate(first_number, operator, second_number);
        let ans = match ans {
            0 => Item::Number(Number::Zero),
            1 => Item::Number(Number::One),
            2 => Item::Number(Number::Two),
            3 => Item::Number(Number::Three),
            4 => Item::Number(Number::Four),
            5 => Item::Number(Number::Five),
            6 => Item::Number(Number::Six),
            7 => Item::Number(Number::Seven),
            8 => Item::Number(Number::Eight),
            9 => Item::Number(Number::Nine),
            _ => Item::Number(Number::Zero), // Default case for unexpected values
        };

        expression[place] = Token {
            item: ans,
            value: 0,
        };

        expression.remove(place + 1);
        expression.remove(place + 1);
    }

    let ans = match &expression[0].item {
        Item::Number(num) => Number::get_value(num.to_owned()),
        _ => 0,
    };

    ans
}

pub fn calculate(first_number: Item, operator: Item, second_number: Item) -> u32 {
    let first_value = match first_number {
        Item::Number(num) => Number::get_value(num),
        _ => 0,
    };

    let second_value = match second_number {
        Item::Number(num) => Number::get_value(num),
        _ => 0,
    };

    match operator {
        Item::Operator(op) => match op {
            Operator::Add => first_value + second_value,
            Operator::Subtract => first_value - second_value,
            Operator::Multiply => first_value * second_value,
            Operator::Divide => first_value / second_value,
        },
        _ => 0,
    }
}