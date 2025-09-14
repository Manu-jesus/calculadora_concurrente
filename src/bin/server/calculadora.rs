use std::{str::FromStr};

// A basic wrapping u8 calculator.
//
// The possible values range from [0;256).
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
        // Split the string into tokens separated by whitespace.
        let tokens: Vec<&str> = s.split_whitespace().collect();

        // Try to convert the vector into a statically-sized array of 2 elements, failing otherwise.
        let [operation, operand] = tokens.try_into().map_err(|_| "expected 2 arguments")?;

        // Parse the operand into an u8.
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
