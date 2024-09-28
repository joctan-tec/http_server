
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use std::process::{Command, Output};
use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

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


/// Analiza la línea de solicitud HTTP y devuelve el método y la ruta
fn parse_request_line(request_line: &str) -> (&str, String) {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return ("", "".to_string());
    }
    (parts[0], parts[1].to_string())
}

fn write_to_temp_file<P: AsRef<Path>>(path: P, content: String) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn delete_temp_file<P: AsRef<Path>>(path: P) -> io::Result<()> {
    std::fs::remove_file(path)
}

fn get_current_dir() -> PathBuf {
    env::current_dir().unwrap()
}



/// Función para ejecutar el script de Python y capturar su salida o error
fn execute_python_script(option: &str, name: &str) -> Result<Value, String> {

    // Obtener el directorio actual del ejecutable
    let current_dir = get_current_dir();
    println!("Directorio actual: {:?}", current_dir);

    // Crear la ruta completa del script
    let script_path: PathBuf = current_dir.join("scripts").join("json_management.py");
    println!("Ejecutando el script de Python en la ruta: {:?}", script_path);


    println!("Ejecutando el script de Python en la ruta: {:?}", script_path);

    // Verificar si el archivo existe
    if !script_path.exists() {
        println!("El script no existe en la ruta: {:?}", script_path);
        return Err("El script de Python no existe".to_string());
    }

    let output: Output = Command::new("python3")
        .arg(script_path) //Path al script de Python
        .arg("--option")
        .arg(option)
        .arg("--name")
        .arg(name)
        .output()
        .expect("Failed to execute Python script");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).map_err(|e| e.to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}

/// Maneja la operación GET para devolver los datos completos del JSON
fn handle_get() -> (&'static str, String) {
    let option = "0"; // Opción para GET
    let dummy_name = ""; // No se necesita el nombre del archivo para GET

    match execute_python_script(option, dummy_name) {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message),
    }
}

/// Maneja la operación POST para crear una nueva escudería
fn handle_post(body: &str) -> (&'static str, String) {
    let option = "1"; // Opción para POST
    let json_request: Value = serde_json::from_str(body).unwrap();
    let current_dir = get_current_dir();
    let file_path = current_dir.join("tmp").join("new_escuderia.json");
    
    // Escribir en el archivo temporal
    write_to_temp_file(&file_path, json_request.to_string()).unwrap();

    // Ejecutar el script de Python
    let result = execute_python_script(option, "new_escuderia.json");
    
    // Eliminar el archivo temporal
    if let Err(err) = delete_temp_file(&file_path) {
        eprintln!("Error al eliminar el archivo temporal: {}", err);
    }

    // Retornar la respuesta según el resultado
    match result {
        Ok(response) => ("HTTP/1.1 201 CREATED", response.to_string()),
        Err(error_message) => ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message),
    }
    
    
}

/// Maneja la operación PUT para actualizar una escudería existente
fn handle_put(path: &str, body: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 4 {
        let error_response = r#"{"error": "Invalid path"}"#;
        return ("HTTP/1.1 400 BAD REQUEST", error_response.to_string());
    }

    let option = "2"; // Opción para PUT
    match execute_python_script(option, body) {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message),
    }
}

/// Maneja la operación DELETE para eliminar una escudería existente
fn handle_delete(path: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 4 {
        let error_response = r#"{"error": "Invalid path"}"#;
        return ("HTTP/1.1 400 BAD REQUEST", error_response.to_string());
    }

    let nombre_escuderia = parts[3].replace("%20", " ");
    let option = "3"; // Opción para DELETE

    match execute_python_script(option, &nombre_escuderia) {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message),
    }
}

/// Maneja la operación PATCH para actualizar un piloto específico
fn handle_patch(path: &str, body: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 6 {
        let error_response = r#"{"error": "Invalid path"}"#;
        return ("HTTP/1.1 400 BAD REQUEST", error_response.to_string());
    }

    let option = "4"; // Opción para PATCH
    match execute_python_script(option, body) {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message),
    }
}

/// Maneja la conexión entrante y dirige la solicitud al método correspondiente
fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();

    if buf_reader.read_line(&mut request_line).is_err() {
        eprintln!("Failed to read request line");
        return;
    }

    println!("Request line: {}", request_line);

    // Leer encabezados
    let mut body = String::new();
    let mut content_length = 0;

    loop {
        let mut header_line = String::new();
        if buf_reader.read_line(&mut header_line).is_err() {
            eprintln!("Failed to read header line");
            return;
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
                return;
            }
        }
    }

    println!("Body: {}", body);

    // Procesar la línea de solicitud
    let (method, path) = parse_request_line(&request_line);
    println!("Method: {}, Path: {}", method, path);

    // Determinar y manejar la solicitud según el método y la ruta
    let (status_line, response_body) = match (method, path.as_str()) {
        ("GET", "/api/escuderias") => {
            // let datos_guard = datos.lock().unwrap();
            // let json_response = serde_json::to_string(&*datos_guard).unwrap();
            // ("HTTP/1.1 200 OK", json_response)
            handle_get()
        }
        ("POST", "/api/escuderias") => {
            handle_post(&body)
        }
        ("PUT", path) if path.starts_with("/api/escuderias/") => {
            handle_put(path, &body)
        }
        ("DELETE", path) if path.starts_with("/api/escuderias/") => {
            handle_delete(path)
        }
        ("PATCH", path) if path.contains("/pilotos/") => {
            handle_patch(path, &body)
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

/// Definición del ThreadPool y sus componentes
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Crea un nuevo ThreadPool con el tamaño especificado
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    /// Ejecuta una nueva tarea en el ThreadPool
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
    /// Crea un nuevo Worker y lo añade al ThreadPool
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
    /// Espera a que todos los hilos finalicen al destruir el ThreadPool
    fn drop(&mut self) {
        // Se deja de enviar trabajos al sender
        drop(self.sender.clone());
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

fn main() {
    // Cargar el JSON desde el archivo
    // let datos = load_json("./f1_data.json").expect("Failed to load JSON data");
    // let datos = Arc::new(Mutex::new(datos)); // Compartir datos de manera segura entre hilos
    

    let pool = ThreadPool::new(20);
    let listener = TcpListener::bind("127.0.0.1:7000").unwrap();

    println!("Server listening on port 7000...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // let datos_clone = Arc::clone(&datos); // Clonar el Arc para pasarlo al hilo
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}