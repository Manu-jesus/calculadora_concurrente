use crate::calculadora::Aritmetic;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Op(Aritmetic),
    Get,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        let operation = *tokens
            .first()
            .ok_or("expected operation as first argument")?;

        match operation {
            "OP" => {
                if tokens.len() < 3 {
                    let message = match tokens.get(1) {
                        Some(value) => value,
                        None => return Err("unknown operation".to_string()),
                    };
                    return Err(format!("unexpected message: {}", message));
                }

                let number_operation: u8 = tokens
                    .get(2)
                    .ok_or("unexpected message: ")?
                    .parse()
                    .map_err(|e: ParseIntError| format!("parsing error: invalid integer: {}", e))?;

                let [_, signe, _] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

                let aritmetic = match signe {
                    "+" => Aritmetic::Add(number_operation),
                    "-" => Aritmetic::Sub(number_operation),
                    "*" => Aritmetic::Mul(number_operation),
                    "/" => {
                        if number_operation == 0 {
                            return Err("division by zero".to_string());
                        }
                        Aritmetic::Div(number_operation)
                    }
                    unknown => return Err(format!("unknown operation: {}", unknown)),
                };

                Ok(Operation::Op(aritmetic))
            }
            "GET" => Ok(Operation::Get),
            _ => Err(String::from("invalid command format")),
        }
    }
}
#[test]
fn test_from_str_op_add() {
    let result = Operation::from_str("OP + 10");
    assert_eq!(result, Ok(Operation::Op(Aritmetic::Add(10))));
}

#[test]
fn test_from_str_op_sub() {
    let result = Operation::from_str("OP - 5");
    assert_eq!(result, Ok(Operation::Op(Aritmetic::Sub(5))));
}

#[test]
fn test_from_str_op_mul() {
    let result = Operation::from_str("OP * 2");
    assert_eq!(result, Ok(Operation::Op(Aritmetic::Mul(2))));
}

#[test]
fn test_from_str_op_div() {
    let result = Operation::from_str("OP / 3");
    assert_eq!(result, Ok(Operation::Op(Aritmetic::Div(3))));
}

#[test]
fn test_from_str_get() {
    let result = Operation::from_str("GET");
    assert_eq!(result, Ok(Operation::Get));
}

#[test]
fn test_from_str_unknown_main_operation() {
    // Este test ahora debería esperar "invalid command format"
    let result = Operation::from_str("UNKNOWN");
    assert_eq!(result, Err(String::from("invalid command format")));
}

#[test]
fn test_from_str_op_invalid_number() {
    let result = Operation::from_str("OP + abc");
    let expected_error = "parsing error: invalid integer: invalid digit found in string";
    assert_eq!(result.unwrap_err(), expected_error);
}

#[test]
fn test_from_str_op_unknown_arithmetic_sign() {
    let result = Operation::from_str("OP ! 10");
    assert_eq!(result, Err(String::from("unknown operation: !")));
}

#[test]
fn test_from_str_op_division_by_zero() {
    let result = Operation::from_str("OP / 0");
    assert_eq!(result, Err(String::from("division by zero")));
}
