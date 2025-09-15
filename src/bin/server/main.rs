use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

mod calculadora;
use calculadora::Calculator;
mod operation;
use operation::Operation;

fn main() {
    let host: String = match parse_arguments() {
        Ok(port) => port,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let service = match enable_host_port(host) {
        Some(value) => value,
        None => return,
    };

    println!("{:?}", service)
}

fn enable_host_port(host: String) -> Option<String> {
    let listener = TcpListener::bind(host).ok()?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                procesar_conexion_por_lineas(&stream);
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }

    Some("El servidor se detuvo.".to_string())
}

fn procesar_conexion_por_lineas(mut stream: &TcpStream) {
    let mut calculator = Calculator::default();

    let reader = BufReader::new(stream);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let operation = match Operation::from_str(&line) {
                    Ok(operation) => operation,
                    Err(error) => {
                        eprintln!("failed to parse line {}", error);
                        continue;
                    }
                };

                match operation {
                    Operation::Op(aritmetic_data) => {
                        calculator.apply(aritmetic_data);
                    }
                    Operation::Get => {
                        let response = format!("{}\n", calculator.value());

                        if let Err(e) = stream.write(response.as_bytes()) {
                            eprintln!("Error al escribir respuesta al cliente: {}", e);
                            break;
                        }
                        if let Err(e) = stream.flush() {
                            eprintln!("Error al hacer flush de la respuesta al cliente: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error al leer la línea: {}", e);
                break;
            }
        }
    }
}

fn parse_arguments() -> Result<String, &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let host_str = inputs
        .next()
        .ok_or("missing port. Usage: cargo run --bin server -- <port>")?;

    Ok(host_str)
}
