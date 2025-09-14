use std::{fmt::Display, str::FromStr};
// mod calculadora;
use crate::{calculadora::Calculator, operation};
use crate::calculadora::Aritmetic;

enum Response {
    /// Respuesta del Contains
    Yes,
    /// Respuesta del Contains
    No,
    ///Respuesta del Insert y Remove exitoso
    Ok,
    /// Respuesta del Get
    Values(Vec<u8>),
    /// Respuesta de cualquier operacion que falle
    Error(String),
}


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

                let [_, signe, _] = tokens
                    .try_into()
                    .map_err(|_| "expected 2 arguments")?;

                let aritmetic = match signe {
                    "+" => Ok(Aritmetic::Add(number_operation)),
                    "-" => Ok(Aritmetic::Sub(number_operation)),
                    "*" => Ok(Aritmetic::Mul(number_operation)),
                    "/" => Ok(Aritmetic::Div(number_operation)),
                    _ => Err("unknown operation"),
                }?;

                println!("el aritmetic: {:?}", aritmetic);

                Ok(Operation::Op(aritmetic))
            
            }
            "GET" => Ok(Operation::Get),
            _ => Err("unknown operation"),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Response::Yes => write!(f, "YES"),
            Response::No => write!(f, "NO"),
            Response::Ok => write!(f, "OK"),
            Response::Values(values) => {
                write!(
                    f,
                    "VALUES {}",
                    values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            Response::Error(reason) => write!(f, "ERROR \"{}\"", reason),
        }
    }
}
