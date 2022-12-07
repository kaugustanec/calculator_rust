use std::str::FromStr;
use text_io::scan;

#[derive(Debug)]
struct ErrorString(String);

impl ErrorString {
    pub fn new(error_type: impl ToString) -> Self {
        Self(error_type.to_string())
    }
}

enum Operation {
    Multiplication,
    Addition,
    Subtraction,
    Division,
}

impl FromStr for Operation {
    type Err = ErrorString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Addition),
            "-" => Ok(Operation::Subtraction),
            ":" => Ok(Operation::Division),
            "*" => Ok(Operation::Multiplication),
            _ => Err(ErrorString(String::from("Invalid operator provided."))),
        }
    }
}

struct Expression {
    first_number: f64,
    second_number: f64,
    operator: Result<Operation, ErrorString>,
}

impl Expression {
    fn new(
        first_number: f64,
        second_number: f64,
        operator: Result<Operation, ErrorString>,
    ) -> Self {
        Expression {
            first_number,
            second_number,
            operator,
        }
    }

    fn match_operation(&self) -> Result<f64, ErrorString> {
        match &self.operator {
            Ok(operator) => match operator {
                Operation::Addition => Ok(self.add_numbers()),
                Operation::Division => Ok(self.divide_numbers()),
                Operation::Multiplication => Ok(self.multiply_numbers()),
                Operation::Subtraction => Ok(self.subtract_numbers()),
            },
            Err(v) => Err(ErrorString(String::from("Error"))),
        }
    }

    fn add_numbers(&self) -> f64 {
        self.first_number + self.second_number
    }

    fn subtract_numbers(&self) -> f64 {
        self.first_number - self.second_number
    }

    fn multiply_numbers(&self) -> f64 {
        self.first_number * self.second_number
    }

    fn divide_numbers(&self) -> f64 {
        self.first_number / self.second_number
    }
}

fn main() {
    let first_number: f64;
    let second_number: f64;
    let operator: String;

    scan!("{} {} {}", first_number, operator, second_number);
    let exp = Expression::new(first_number, second_number, Operation::from_str(&operator));

    println!("{:?}", exp.match_operation())
}
