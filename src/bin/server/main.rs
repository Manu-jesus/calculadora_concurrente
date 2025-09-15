use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
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
        Ok(value) => value,
        Err(_) => return,
    };

    println!("{:?}", service)
}

fn enable_host_port(host: String) -> Result<String, ()> {
    let listener = match TcpListener::bind(host) {
        Ok(listen) => listen,
        Err(_) => return Err(()),
    };

    let calculator = Arc::new(Mutex::new(Calculator::default()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let cpy = match stream.try_clone() {
                    Ok(wrt) => wrt,
                    Err(_) => return Err(()),
                };

                let calculator_arc = Arc::clone(&calculator);

                thread::spawn(move || {
                    let _ = procesar_conexion_por_lineas(stream, cpy, calculator_arc);
                });
            }
            Err(e) => {
                eprintln!("Error al aceptar la conexión: {}", e);
            }
        }
    }

    Ok("OK".to_string())
}

fn procesar_conexion_por_lineas(
    stream: TcpStream,
    mut write_stream: TcpStream,
    calculator: Arc<Mutex<Calculator>>,
) -> std::io::Result<()> {
    let reader = BufReader::new(stream);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let operation = match Operation::from_str(&line) {
                    Ok(operation) => operation,
                    Err(error) => {
                        let error_message = format!("ERROR \"{}\"\n", error);
                        //eprintln!("{}", error_message);
                        write_stream.write_all(error_message.as_bytes())?;
                        continue;
                    }
                };

                let mut calculator_locket = match calculator.lock() {
                    Ok(calc) => calc,
                    Err(e) => {
                        eprintln!("Error al bloquear el mutex: {}", e);
                        break;
                    }
                };

                match operation {
                    Operation::Op(aritmetic_data) => {
                        calculator_locket.apply(aritmetic_data);
                        write_stream.write_all("OK\n".as_bytes())?;
                    }
                    Operation::Get => {
                        let response = format!("{}\n", calculator_locket.value());

                        write_stream.write_all(response.as_bytes())?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error al leer la línea: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn parse_arguments() -> Result<String, &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let host_str = inputs
        .next()
        .ok_or("missing port. Usage: cargo run --bin server -- <port>")?;

    Ok(host_str)
}
