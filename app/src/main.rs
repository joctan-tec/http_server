mod utils;
mod json_hashmaps;
mod http_functions;
mod server_http;

use server_http::server::Server;
use std::{io::Write, net::TcpStream};
use json_hashmaps::f1_data_hashmap::get_f1_data;




fn main() {
    let mut server = Server::new(20); // Crear el servidor con un pool de 20 hilos

    // Definir las rutas y métodos
    server.add_route("GET", "/", |stream: &mut TcpStream, _request: &str| {
        std::thread::sleep(std::time::Duration::from_secs(7));
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Success!\"}";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    server.add_route("GET", "/data", |stream: &mut TcpStream, _request: &str| {
        let f1_data = get_f1_data().unwrap();
        let pretty_json = serde_json::to_string_pretty(&f1_data).unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}", pretty_json);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    server.start("0.0.0.0", 7000); // Iniciar el servidor
}