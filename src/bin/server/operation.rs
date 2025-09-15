use crate::calculadora::Aritmetic;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub enum Operation {
    Op(Aritmetic),
    Get,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        let operation = *tokens
            .first()
            .ok_or("expected operation as first argument")?;

        match operation {
            "OP" => {
                let number_operation: u8 = tokens
                    .get(2)
                    .ok_or("expected number as third argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                let [_, signe, _] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

                let aritmetic = match signe {
                    "+" => Ok(Aritmetic::Add(number_operation)),
                    "-" => Ok(Aritmetic::Sub(number_operation)),
                    "*" => Ok(Aritmetic::Mul(number_operation)),
                    "/" => Ok(Aritmetic::Div(number_operation)),
                    _ => Err("unknown operation"),
                }?;

                Ok(Operation::Op(aritmetic))
            }
            "GET" => Ok(Operation::Get),
            _ => Err("unknown operation"),
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
    let result = Operation::from_str("UNKNOWN");
    assert_eq!(result, Err("unknown operation"));
}

#[test]
fn test_from_str_empty_string() {
    let result = Operation::from_str("");
    assert_eq!(result, Err("expected operation as first argument"));
}

#[test]
fn test_from_str_op_with_missing_arguments() {
    let result = Operation::from_str("OP +");
    assert_eq!(result, Err("expected number as third argument"));
}

#[test]
fn test_from_str_op_invalid_number() {
    let result = Operation::from_str("OP + abc");
    assert_eq!(result, Err("failed to parse number"));
}

#[test]
fn test_from_str_op_unknown_arithmetic_sign() {
    let result = Operation::from_str("OP ! 10");
    assert_eq!(result, Err("unknown operation"));
}
