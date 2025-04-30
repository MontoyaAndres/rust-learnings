use regex::Regex;
use std::str::FromStr;

const ADDITION: &str = "(\\d+)\\s*?(\\+)\\s*?(\\d+)";
const SUBTRACTION: &str = "(\\d+)\\s*?(\\-)\\s*?(\\d+)";
const MULTIPLICATION: &str = "(\\d+)\\s*?(\\*)\\s*?(\\d+)";
const DIVISION: &str = "(\\d+)\\s*?(/)\\s*?(\\d+)";

fn operations(mut expression: String, operator: &str) -> String {
    let re = Regex::from_str(operator).unwrap();

    loop {
        let caps = re.captures(&expression);

        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let operator = caps.get(2).unwrap().as_str();
        let right_value: i32 = caps.get(3).unwrap().as_str().parse().unwrap();

        let result = match operator {
            "+" => left_value + right_value,
            "*" => left_value * right_value,
            "-" => left_value - right_value,
            "/" => left_value / right_value,
            _ => panic!("Unknown operator"),
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }

    expression.to_string()
}

fn main() {
    println!("Enter expression:");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    expression = operations(expression, ADDITION);
    expression = operations(expression, MULTIPLICATION);
    expression = operations(expression, SUBTRACTION);
    expression = operations(expression, DIVISION);

    println!("Result: {}", expression);
}
