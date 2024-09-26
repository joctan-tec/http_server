use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Piloto {
    nombre: String,
    edad: u32,
    nacionalidad: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Escuderia {
    nombre: String,
    pilotos: Vec<Piloto>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Datos {
    escuderias: Vec<Escuderia>,
}

/// Carga el archivo JSON y devuelve los datos deserializados
fn load_json(filename: &str) -> Result<Datos, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filename)?;
    let datos: Datos = serde_json::from_str(&data)?;
    Ok(datos)
}

fn handle_connection(mut stream: TcpStream, datos: &Datos) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    
    if buf_reader.read_line(&mut request_line).is_err() {
        eprintln!("Failed to read request line");
        return; // Salir si no se pudo leer la línea de solicitud
    }

    println!("Request line: {}", request_line); // Log de la línea de solicitud

    // Leer encabezados
    let mut body = String::new();
    let mut content_length = 0;

    loop {
        let mut header_line = String::new();
        if buf_reader.read_line(&mut header_line).is_err() {
            eprintln!("Failed to read header line");
            return; // Salir si no se pudo leer una línea de encabezado
        }

        // Fin de los encabezados: línea vacía
        if header_line == "\r\n" {
            break;
        }

        // Obtener Content-Length si está presente
        if header_line.starts_with("Content-Length:") {
            if let Some(length_str) = header_line.split(':').nth(1) {
                content_length = length_str.trim().parse::<usize>().unwrap_or(0);
            }
        }
    }

    // Leer el cuerpo solo si se espera un contenido
    if content_length > 0 {
        let mut additional_body = vec![0; content_length];
        match buf_reader.read_exact(&mut additional_body) {
            Ok(_) => body.push_str(&String::from_utf8_lossy(&additional_body)),
            Err(e) => {
                eprintln!("Failed to read body: {}", e);
                return; // Salimos si no se pudo leer el cuerpo
            }
        }
    }

    println!("Body: {}", body); // Log del cuerpo de la solicitud

    // Procesar la línea de solicitud
    let (method, path) = parse_request_line(&request_line);
    println!("Method: {}, Path: {}", method, path); // Log del método y la ruta

    // Variable para almacenar la respuesta
    let (status_line, response_body) = match (method, path) {
        ("GET", "/api/escuderias") => {
            let json_response = serde_json::to_string(datos).unwrap();
            ("HTTP/1.1 200 OK", json_response)
        }
        ("POST", "/api/test") => {
            if body.is_empty() {
                let error_response = r#"{"error": "Empty body"}"#;
                let length = error_response.len();
                let response = format!(
                    "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{error_response}"
                );
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Failed to write response: {}", e);
                }
                return; // Salimos después de enviar la respuesta
            }

            match serde_json::from_str::<Value>(&body) {
                Ok(json_body) => {
                    ("HTTP/1.1 200 OK", json_body.to_string())
                }
                Err(_) => {
                    let error_response = r#"{"error": "Invalid JSON"}"#;
                    let length = error_response.len();
                    let response = format!(
                        "HTTP/1.1 400 BAD REQUEST\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{error_response}"
                    );
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        eprintln!("Failed to write response: {}", e);
                    }
                    return;
                }
            }
        }
        _ => {
            let error_response = r#"{"error": "Not Found"}"#;
            ("HTTP/1.1 404 NOT FOUND", error_response.to_string())
        }
    };

    let length = response_body.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/json\r\n\r\n{response_body}"
    );

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write response: {}", e);
    }
}

fn parse_request_line(request_line: &str) -> (&str, &str) {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return ("", ""); // Manejo de error básico
    }
    (parts[0], parts[1]) // Método y ruta
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).expect("Failed to send job to the thread pool");
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Se deja de enviar trabajos al sender
        drop(self.sender.clone()); // Clonamos el sender en lugar de moverlo
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    // Cargar el JSON desde el archivo
    let datos = load_json("./f1_data.json").expect("Failed to load JSON data");

    let pool = ThreadPool::new(20);
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    println!("Server listening on port 7000...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let datos_clone = datos.clone(); // Clonamos los datos para cada hilo
        pool.execute(move || {
            handle_connection(stream, &datos_clone);
        });
    }
}
