use std::str::FromStr;
use text_io::scan;

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
    first_number: u32,
    second_number: u32,
    operator: Result<Operation, ErrorString>,
}

impl Expression {
    fn new(
        first_number: u32,
        second_number: u32,
        operator: Result<Operation, ErrorString>,
    ) -> Self {
        Expression {
            first_number,
            second_number,
            operator,
        }
    }

    fn match_operation(&self) -> u32 {
        match &self.operator {
            Ok(operator) => match operator {
                Operation::Addition => self.add_numbers(),
                Operation::Division => self.divide_numbers(),
                Operation::Multiplication => self.multiply_numbers(),
                Operation::Subtraction => self.subtract_numbers(),
            },
            Err(v) => 999,
        }
    }

    fn add_numbers(&self) -> u32 {
        self.first_number + self.second_number
    }

    fn subtract_numbers(&self) -> u32 {
        self.first_number - self.second_number
    }

    fn multiply_numbers(&self) -> u32 {
        self.first_number * self.second_number
    }

    fn divide_numbers(&self) -> u32 {
        self.first_number / self.second_number
    }
}

fn main() {
    let first_number: u32;
    let second_number: u32;
    let operator: String;

    scan!("{} {} {}", first_number, operator, second_number);
    let exp = Expression::new(first_number, second_number, Operation::from_str(&operator));

    println!("{:?}", exp.match_operation())
}
