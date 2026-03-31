use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let addr = "127.0.0.1:3030";
    let listener = TcpListener::bind(addr).expect("No se pudo conectar con el puerto 3030");

    println!("Servidor HTTP escuchando en http//{}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // Cada request se procesa en un hilo independiente para no bloquear el servidor
                thread::spawn( move || {
                    handle_connection(s);
                });
            }
            Err(e) => eprintln!("Error al aceptar conexión: {}", e)
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);
        
        // Extraemos la primera línea (ej: "GET /pi/1000 HTTP/1.1")
        let request_line = request.lines().next().unwrap_or("");
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        // Validamos que sea un GET y la ruta correcta
        if parts.len() >= 2 && parts[0] == "GET" && parts[1].starts_with("/pi/") {
            let i_param = parts[1].trim_start_matches("/pi/");
            
            if let Ok(i) = i_param.parse::<u64>() {
                let resultado = calcular_pi_leibniz(i);
                enviar_respuesta(stream, format!("{:.15}", resultado));
                return;
            }
        }
        
        enviar_respuesta(stream, "Ruta no encontrada o parámetro inválido".to_string());
    }
}

/// Implementación de la Serie de Leibniz
/// pi = 4 * (1/1 - 1/3 + 1/5 - 1/7 + ...)
fn calcular_pi_leibniz(i: u64) -> f64 {
    let mut acumulado = 0.0;
    
    for n in 0..=i {
        let denominador = (2 * n + 1) as f64;
        if n % 2 == 0 {
            acumulado += 1.0 / denominador;
        } else {
            acumulado -= 1.0 / denominador;
        }
    }
    
    acumulado * 4.0
}

fn enviar_respuesta(mut stream: TcpStream, mensaje: String) {
    let respuesta = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        mensaje.len(),
        mensaje
    );
    let _ = stream.write_all(respuesta.as_bytes());
}
