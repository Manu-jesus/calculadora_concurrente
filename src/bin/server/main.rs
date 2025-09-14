use std::{fmt::{Display}, net::{TcpListener, TcpStream}, str::FromStr, io::{BufReader, BufRead}, fs::File};

mod calculadora;
use calculadora::Calculator;
use calculadora::Aritmetic;
mod operation;
use operation::Operation;

fn main() {
    let port: u16 = match parse_arguments() {
        Ok(port) => port,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    

    //enable_host_port(port);
    let service = match enable_host_port(port) {
        Some(value) => value,
        None => return,
    };
    
    println!("{:?}", service)
}


fn enable_host_port(port: u16) -> Option<String> {

    println!("El servidor está escuchando en el puerto {}", port);

    let localhots = "127.0.0.1:".to_string() + &port.to_string();
    
    let listener = TcpListener::bind(localhots).ok()?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                procesar_conexion_por_lineas(stream);

            },
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }

    Some(format!("El servidor en el puerto {} se detuvo.", port))

}

fn procesar_conexion_por_lineas(stream: TcpStream) {

    // We maintain a *global* calculator for the entire program.
    let mut calculator = Calculator::default();
    println!("¡Conexión establecida! con {:?}", stream);

    let reader = BufReader::new(stream);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                // Aquí, cada `line` es un String que no incluye el salto de línea.
                println!("Línea leída: {}", line);
                
                let operation = match Operation::from_str(&line) {
                    Ok(operation) => operation,
                    Err(error) => {
                        eprintln!("failed to parse line {}", error);
                        continue;
                    }
                };

                match operation {
                    Operation::Op(aritmetic_data) => {
                        println!("Procesando operación aritmética: {:?}", aritmetic_data);
                    
                        calculator.apply(aritmetic_data);
                        println!("Resultado de la calculadora: {}", calculator.value());
                    }
                    Operation::Get => {
                        println!("ahora vale: {:?}", calculator.value());
                    }
                };

                
                println!("{:?}", calculator.value());
            },
            Err(e) => {
                eprintln!("Error al leer la línea: {}", e);
                break; // Termina el bucle si hay un error de I/O.
            }
        }
    }
}

// NO recomendamos usar strings para los errores de su programa.
// Recomendamos enums, lo hacemos por temas de tiempo ;)
fn parse_arguments() -> Result<u16, &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let port_str = inputs
        .next()
        .ok_or("missing port. Usage: cargo run --bin server -- <port>")?;

    let port: u16 = port_str.parse().map_err(|_| "port must be a valid u16")?;

    Ok(port)
}

// #[cfg(test)]
// mod test {
//     use std::str::FromStr;

//     use crate::{Operation, Response};

//     #[test]
//     fn parse_operation() {
//         let cases = [
//             ("INSERT 10", Operation::Insert(10)),
//             ("CONTAINS 10", Operation::Contains(10)),
//             ("REMOVE 10", Operation::Remove(10)),
//             ("GET", Operation::Get),
//         ];

//         for (operation_string, expected_operation) in cases {
//             let operation = Operation::from_str(operation_string).unwrap();
//             assert_eq!(operation, expected_operation)
//         }
//     }

//     #[test]
//     fn print_response() {
//         let cases = [
//             (Response::Ok, "OK"),
//             (Response::Yes, "YES"),
//             (Response::No, "NO"),
//             (Response::Values(vec![1, 2, 3]), "VALUES 1 2 3"),
//             (
//                 Response::Error(String::from("failure")),
//                 "ERROR \"failure\"",
//             ),
//         ];

//         for (response, expected_response_string) in cases {
//             let response_string = response.to_string();
//             assert_eq!(response_string, expected_response_string)
//         }
//     }
// }
