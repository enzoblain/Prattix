use crate::{Item, Operator, Token};

pub fn evaluate(mut expression: Vec<Token>) -> f64 {
    // We loop until we have only one token left in the expression
    // This token will be the result of the expression
    while expression.len() > 1 {
        // This stores the place of the operator with the highest precedence
        // and the value of that operator
        let mut place = 0;
        let mut value = expression[1].value;

        // We start with the first operator and the first two numbers as default
        let mut first_number = expression[0].item.clone();
        let mut operator = expression[1].item.clone();
        let mut second_number = expression[2].item.clone();

        // We iterate through the expression to find the operator with the highest precedence
        // We start from the third token because the first two are already set
        for i in 3..expression.len() {
            let item = &expression[i];

            // We skip the numbers
            if item.value == 0 {
                continue;
            }

            if value == 2 {
                // If we already have an operator with the highest precedence,
                // we break the loop to avoid unnecessary checks
                break;
            }

            // If the current operator has a higher precedence than the previous one,
            // we update the place and value
            // We also update the numbers and the operator
            if item.value > value {
                place = i - 1;
                value = item.value;
                first_number = expression[i - 1].item.clone();
                operator = item.item.clone();
                second_number = expression[i + 1].item.clone();
            }
        }

        // Then we calculate the result of the operation
        let ans = calculate(first_number, operator, second_number);

        // And we replace the operator and the numbers with result
        expression[place] = Token {
            item: Item::Number(ans),
            value: 0,
        };
        expression.remove(place + 1);
        expression.remove(place + 1);
    }

    // Finally, we return the result of the expression
    // The result is the only token left in the expression
    match expression[0].item {
        Item::Number(num) => num,
        _ => 0.0,
    }
}

// This function calculates the result of an operation
pub fn calculate(first_number: Item, operator: Item, second_number: Item) -> f64 {
    // We match the operator and perform the corresponding operation

    let first_value = match first_number {
        Item::Number(num) => num,
        _ => 0.0,
    };

    let second_value = match second_number {
        Item::Number(num) => num,
        _ => 0.0,
    };

    match operator {
        Item::Operator(op) => match op {
            Operator::Add => first_value + second_value,
            Operator::Subtract => first_value - second_value,
            Operator::Multiply => first_value * second_value,
            Operator::Divide => first_value / second_value,
        },
        _ => 0.0,
    }
}