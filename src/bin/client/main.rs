use std::{
    env::Args,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

fn main() {
    let (address, operation) = match parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    match process_information(operation, &address) {
        Ok(_) => (),
        Err(_) => {
            println!("terminó el procesocon Error");
        }
    };
}

fn process_information(inputs: std::env::Args, address: &String) -> Result<(), ()> {
    for input in inputs {
        let file = match File::open(input) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("ERROR : No se puede abrir el archivo");
                return Err(());
            }
        };

        let file_reader = BufReader::new(file);

        if let Ok(mut stream) = TcpStream::connect(address) {
            if let Some(value) = proccess_lines(file_reader, stream, address) {
                return value;
            }
        } else {
            println!("No se pudo conectar...");
        };
    }

    Ok(())
}

fn proccess_lines(
    file_reader: BufReader<File>,
    mut stream: TcpStream,
    _: &String,
) -> Option<Result<(), ()>> {
    for line in file_reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("failed to read line {}", error);
                break;
            }
        };

        let op = String::from("OP");
        let request = format!("{} {}\n", op, line);

        match stream.write(request.as_bytes()) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error wirte.");
                return Some(Err(()));
            }
        };

        match stream.flush() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error in flush");
                return Some(Err(()));
            }
        };

        let copy = match stream.try_clone() {
            Ok(wrt) => wrt,
            Err(_) => return Some(Err(())),
        };


        let mut reader = BufReader::new(copy);
        let mut response = String::new();

        let _ = match reader.read_line(&mut response) {
            Ok(_) => Ok(response.trim().to_string()),
            Err(err) => Err(format!("Failed to read from stream: {}", err)),
        };
        
        println!("{}", response);

        let tokens: Vec<&str> = response.split_whitespace().collect();
        let operation = *tokens
            .first()
            .ok_or("expected operation as first argument")
            .ok()?;
        match operation {
            "ERROR" => {
                eprintln!("{}", response);
            }
            "VALUE" => {
                eprintln!("{:?}", tokens.get(1));
            }
            _ => {}
        }
    }

    let end_value = String::from("GET\n");

    match stream.write(end_value.as_bytes()) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error wirte.");
            return Some(Err(()));
        }
    };

    match stream.flush() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error in flush");
            return Some(Err(()));
        }
    };

    let mut reader = BufReader::new(stream);
    let mut response = String::new();

    let _ = match reader.read_line(&mut response) {
        Ok(_) => Ok(response.trim().to_string()),
        Err(err) => Err(format!("Failed to read from stream: {}", err)),
    };

    let tokens: Vec<&str> = response.split_whitespace().collect();
    let operation = *tokens
        .first()
        .ok_or("expected operation as first argument")
        .ok()?;
    match operation {
        "ERROR" => {
            eprintln!("{}", response);
        }
        "VALUE" => {
            eprintln!("{:?}", tokens.get(1));
        }
        _ => {}
    }

    None
}

fn parse_arguments() -> Result<(String, Args), &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let address = inputs
        .next()
        .ok_or("missing address. Usage: cargo run --bin client -- <address> <operation>")?;

    Ok((address, inputs))
}
