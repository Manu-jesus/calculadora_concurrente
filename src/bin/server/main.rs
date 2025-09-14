use std::{fmt::{Display}, net::{TcpListener, TcpStream}, str::FromStr, io::{BufReader, BufRead}};

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Insert(u8),
    Contains(u8),
    Remove(u8),
    Get,
}

#[derive(PartialEq, Eq, Debug)]
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

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_whitespace().collect();

        let operation = *tokens
            .first()
            .ok_or("expected operation as first argument")?;

        match operation {
            "INSERT" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Insert(operand))
            }
            "CONTAINS" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Contains(operand))
            }
            "REMOVE" => {
                let operand: u8 = tokens
                    .get(1)
                    .ok_or("expected number as second argument")?
                    .parse()
                    .map_err(|_| "failed to parse number")?;

                Ok(Operation::Remove(operand))
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

fn main() {
    let _ = Operation::from_str("INSERT 10").unwrap();
    let _ = Response::Ok.to_string();
    let _ = Response::Yes.to_string();
    let _ = Response::No.to_string();
    let _ = Response::Values(vec![1, 2, 3]).to_string();
    let _ = Response::Error(String::from("failure")).to_string();

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

    println!("El servidor está bindeando en el puerto {}", port);

    let localhots = "127.0.0.1:".to_string() + &port.to_string();
    
    let listener = TcpListener::bind(localhots).ok()?;

    //Cada stream representa una conexión abierta entre el cliente y el servidor.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // println!("¡Conexión establecida! con {:?}", stream);
                // // No regresamos aquí, el bucle continúa.
                // let mut buffer = [0; 1024];
                // let bytes_read = stream.read(&mut buffer).unwrap();

                // let bytes_validos = &buffer[..bytes_read];
                
                // let peticion = String::from_utf8_lossy(bytes_validos);

                // println!("Se obtubo el siguiente buffer: {:?}", peticion);

                procesar_conexion_por_lineas(stream);

            },
            Err(e) => {
                // Si la conexión entrante falla, puedes manejar el error.
                eprintln!("Error al aceptar la conexión: {}", e);
                // Si quieres que el servidor termine con el error, podrías hacer un 'break'
                // y luego regresar un valor en la siguiente línea.
            }
        }
    }

    Some(format!("El servidor en el puerto {} se detuvo.", port))
    //let mut buffer = [0; 1024];
    //stream.read(&mut buffer).unwrap();

}

fn procesar_conexion_por_lineas(stream: TcpStream) {
    println!("¡Conexión establecida! con {:?}", stream);

    let reader = BufReader::new(stream);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                // Aquí, cada `line` es un String que no incluye el salto de línea.
                println!("Línea leída: {}", line);
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

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{Operation, Response};

    #[test]
    fn parse_operation() {
        let cases = [
            ("INSERT 10", Operation::Insert(10)),
            ("CONTAINS 10", Operation::Contains(10)),
            ("REMOVE 10", Operation::Remove(10)),
            ("GET", Operation::Get),
        ];

        for (operation_string, expected_operation) in cases {
            let operation = Operation::from_str(operation_string).unwrap();
            assert_eq!(operation, expected_operation)
        }
    }

    #[test]
    fn print_response() {
        let cases = [
            (Response::Ok, "OK"),
            (Response::Yes, "YES"),
            (Response::No, "NO"),
            (Response::Values(vec![1, 2, 3]), "VALUES 1 2 3"),
            (
                Response::Error(String::from("failure")),
                "ERROR \"failure\"",
            ),
        ];

        for (response, expected_response_string) in cases {
            let response_string = response.to_string();
            assert_eq!(response_string, expected_response_string)
        }
    }
}
