use std::{net::TcpStream, io::Write};

fn main() {
    let (address, operation) = match parse_arguments() {
        Ok(args) => args,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    println!(
        "Me tengo que conectar con el servidor en {} y enviarle el comando {}",
        address, operation
    );

    let op_1 = "OP + 42\n";
    let op_2 = "OP - 10\n";
    let op_3 = "OP + 1\n";
    let get = "GET\n";
    let op_4 = "OP / 11\n";
    let op_5 = "OP * 2\n";

    let list = [op_1, op_2, op_3, get, op_4, op_5];

    if let Ok(mut stream) = TcpStream::connect(address) {

        println!("Connectado al servidor!");
        
        for request in list {
            let write = match stream.write(request.as_bytes()){
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Error wirte.");
                    return
                }
            };
            println!("write resp: {:?}", write);
    
            let _ = match stream.flush() {
                Ok(value) => value,
                Err(_) => {
                    eprintln!("Error in flush");
                    return
                }
            };
            
        }
    
    } else {
        println!("No se pudo conectar...");
    };


}


// *NO* recomendamos usar strings para los errores de su programa.
// Recomendamos enums, lo hacemos por temas de tiempo ;)
fn parse_arguments() -> Result<(String, String), &'static str> {
    let mut inputs = std::env::args();

    inputs.next();

    let address = inputs
        .next()
        .ok_or("missing address. Usage: cargo run --bin client -- <address> <operation>")?;
    let operation = inputs
        .next()
        .ok_or("missing operation. Usage: cargo run --bin client -- <address> <operation>")?;

    Ok((address, operation))
}

// fn parse_address(address: &String) -> Option<(Vec<u32>, u16)>{
    
//     let (ip_str, port_str) = match address.split_once(':') {
//             Some(valor) => valor,
//             None => {
//                 println!("No se encontró ningún valor.");
//                 return None
//             }
//         };
    
//     // Convertir los segmentos de la IP a números (u32).
//     let ip_segments_numeros  = ip_str.split('.').map(|s| s.parse::<u32>());

//     // Convertir el puerto a un número (u16).
//     let port_number = port_str.parse::<u16>().ok()?;

//     return Some((ip_segments_numeros, port_number))
// }
