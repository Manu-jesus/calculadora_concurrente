use std::str::FromStr;

#[derive(Default)]
pub struct Calculator {
    value: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Aritmetic {
    Add(u8),
    Sub(u8),
    Mul(u8),
    Div(u8),
}

impl Calculator {
    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn apply(&mut self, op: Aritmetic) {
        match op {
            Aritmetic::Add(operand) => self.value = self.value.wrapping_add(operand),
            Aritmetic::Sub(operand) => self.value = self.value.wrapping_sub(operand),
            Aritmetic::Mul(operand) => self.value = self.value.wrapping_mul(operand),
            Aritmetic::Div(operand) => self.value = self.value.wrapping_div(operand),
        }
    }
}

impl FromStr for Aritmetic {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        let operand: u8 = operand.parse().map_err(|_| "operand is not an u8")?;

        match operation {
            "+" => Ok(Aritmetic::Add(operand)),
            "-" => Ok(Aritmetic::Sub(operand)),
            "*" => Ok(Aritmetic::Mul(operand)),
            "/" => Ok(Aritmetic::Div(operand)),
            _ => Err("unknown operation"),
        }
    }
}

#[test]
fn test_value_initial() {
    let calculator = Calculator::default();
    assert_eq!(calculator.value(), 0);
}

#[test]
fn test_addition() {
    let mut calculator = Calculator::default();
    calculator.apply(Aritmetic::Add(5));
    assert_eq!(calculator.value(), 5);
}

#[test]
fn test_subtraction() {
    let mut calculator = Calculator::default();
    calculator.apply(Aritmetic::Add(10));
    calculator.apply(Aritmetic::Sub(3));
    assert_eq!(calculator.value(), 7);
}

#[test]
fn test_multiplication() {
    let mut calculator = Calculator::default();
    calculator.apply(Aritmetic::Add(2));
    calculator.apply(Aritmetic::Mul(4));
    assert_eq!(calculator.value(), 8);
}

#[test]
fn test_division() {
    let mut calculator = Calculator::default();
    calculator.apply(Aritmetic::Add(20));
    calculator.apply(Aritmetic::Div(5));
    assert_eq!(calculator.value(), 4);
}

#[test]
fn test_chained_operations() {
    let mut calculator = Calculator::default();
    calculator.apply(Aritmetic::Add(10));
    calculator.apply(Aritmetic::Mul(2));
    calculator.apply(Aritmetic::Sub(5));
    calculator.apply(Aritmetic::Div(3));
    assert_eq!(calculator.value(), 5);
}

#[test]
fn test_wrapping_arithmetic() {
    let mut calculator = Calculator { value: 250 };
    calculator.apply(Aritmetic::Add(10));
    assert_eq!(calculator.value(), 4);
}

#[test]
fn test_from_str_add() {
    let result = Aritmetic::from_str("+ 10").unwrap();
    assert_eq!(result, Aritmetic::Add(10));
}

#[test]
fn test_from_str_sub() {
    let result = Aritmetic::from_str("- 5").unwrap();
    assert_eq!(result, Aritmetic::Sub(5));
}

#[test]
fn test_from_str_mul() {
    let result = Aritmetic::from_str("* 2").unwrap();
    assert_eq!(result, Aritmetic::Mul(2));
}

#[test]
fn test_from_str_div() {
    let result = Aritmetic::from_str("/ 3").unwrap();
    assert_eq!(result, Aritmetic::Div(3));
}

#[test]
fn test_from_str_invalid_operation() {
    let result = Aritmetic::from_str("! 10");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "unknown operation");
}

#[test]
fn test_from_str_missing_operand() {
    let result = Aritmetic::from_str("+");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "expected 2 arguments");
}

#[test]
fn test_from_str_invalid_operand() {
    let result = Aritmetic::from_str("+ abc");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "operand is not an u8");
}

#[test]
fn test_from_str_empty_string() {
    let result = Aritmetic::from_str("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "expected 2 arguments");
}
