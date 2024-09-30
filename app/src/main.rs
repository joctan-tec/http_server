
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use serde::{Deserialize, Serialize};
mod thread_pool;
use thread_pool::ThreadPool;
use std::process::{Command, Output};
use std::env;
use serde_json::{json, Value};
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::fs::metadata;

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

fn execute_python_script(option: &str, name: &str) -> Result<Value, String> {
    // Execute the Python script, return either the script changed or an error 
    // Get the current directory of the .py
    let current_dir = get_current_dir();
    println!("Current directory: {:?}", current_dir);

    // Get the full path of the .py
    let script_path: PathBuf = current_dir.join("scripts").join("json_management.py");
    println!("Executing python script in path: {:?}", script_path);

    // Check if the .py exists 
    if !metadata(&script_path).is_ok() { //Using metadata instead of path in case the file exists but it's corrupt 
        let error_message = format!("Python script not found in path: {:?}", script_path);
        println!("{}", error_message);
        return Err(error_message);
    }

    // Execute the python script 
    // Command: Use Python3 to execute python script (path) with an option (the method PUT/GET...) and name (json)
    let output: Output = Command::new("python3") 
        .arg(&script_path)
        .arg("--option")
        .arg(option)
        .arg("--name")
        .arg(name)
        .output()
        .map_err(|e| format!("Error executing python script: {}", e))?;

    // Check the output: Print execution errors
    if output.status.success() {
        // Check if there was an error reading the JSON
        let stdout = String::from_utf8_lossy(&output.stdout);
        serde_json::from_str(&stdout).map_err(|e| format!("Error al parsear JSON: {}", e))
    } else {
        // If not it was an error executing the script
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Error en la ejecución del script: {}", stderr))
    }
}

fn handle_get() -> (&'static str, String) {
    // Get returns all data from the server: teams, drivers and driver's data
    // Doesn't receive anything
    // Executes the python script that fetches the JSON 
    // Prints the error found 
    match execute_python_script("0", "") { 
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => {
            eprintln!("Error al ejecutar el script: {}", error_message);
            ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message)
        },
    }
}

fn handle_post(body: &str) -> (&'static str, String) {
    // Method to write a team into JSON
    // Intentar parsear el cuerpo de la solicitud a JSON
    let json_request: Value = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(_) => return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid JSON body"}"#.to_string()),
    };

    let current_dir = get_current_dir();
    let file_path = current_dir.join("tmp").join("new_escuderia.json");

    // Escribir en el archivo temporal
    if let Err(e) = write_to_temp_file(&file_path, json_request.to_string()) {
        eprintln!("Error al escribir en el archivo temporal: {}", e);
        return ("HTTP/1.1 500 INTERNAL SERVER ERROR", r#"{"error": "Failed to write temp file"}"#.to_string());
    }

    // Ejecutar el script de Python
    let result = execute_python_script("1", "new_escuderia.json");

    // Eliminar el archivo temporal
    if let Err(err) = delete_temp_file(&file_path) {
        eprintln!("Error al eliminar el archivo temporal: {}", err);
    }

    match result {
        Ok(response) => ("HTTP/1.1 201 CREATED", response.to_string()),
        Err(error_message) => {
            eprintln!("Error al ejecutar el script: {}", error_message);
            ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message)
        },
    }
}

fn handle_put(path: &str, body: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 4 {
        return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid path"}"#.to_string());
    }

    let team_name = parts[3];

    // Intentar parsear el cuerpo de la solicitud a JSON
    let body_json: Value = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(_) => return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid JSON body"}"#.to_string()),
    };

    // Crear el JSON final
    let json_request = json!({
        "body": body_json,
        "team": team_name
    });

    let current_dir = get_current_dir();
    let file_path = current_dir.join("tmp").join("updated_escuderia.json");

    // Escribir en el archivo temporal
    if let Err(e) = fs::write(&file_path, json_request.to_string()) {
        eprintln!("Error al escribir en el archivo temporal: {}", e);
        return ("HTTP/1.1 500 INTERNAL SERVER ERROR", r#"{"error": "Failed to write temp file"}"#.to_string());
    }

    // Ejecutar el script de Python
    let result = execute_python_script("2", "updated_escuderia.json");

    // Eliminar el archivo temporal
    if let Err(err) = fs::remove_file(&file_path) {
        eprintln!("Error al eliminar el archivo temporal: {}", err);
    }

    match result {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => {
            eprintln!("Error al ejecutar el script: {}", error_message);
            ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message)
        },
    }
}


fn handle_delete(path: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 4 {
        return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid path"}"#.to_string());
    }

    let team_name = parts[3].replace("%20", " ");

    // Crear el JSON que contiene el nombre del equipo a eliminar
    let json_request = json!({
        "team": team_name
    });

    let current_dir = get_current_dir();
    let file_path = current_dir.join("tmp").join("delete_escuderia.json");

    // Escribir el JSON en el archivo temporal
    if let Err(e) = fs::write(&file_path, json_request.to_string()) {
        eprintln!("Error al escribir en el archivo temporal: {}", e);
        return ("HTTP/1.1 500 INTERNAL SERVER ERROR", r#"{"error": "Failed to write temp file"}"#.to_string());
    }

    // Ejecutar el script de Python
    let result = execute_python_script("3", "delete_escuderia.json");

    // Eliminar el archivo temporal
    if let Err(err) = fs::remove_file(&file_path) {
        eprintln!("Error al eliminar el archivo temporal: {}", err);
    }
    
    match result {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => {
            eprintln!("Error al ejecutar el script: {}", error_message);
            ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message)
        },
    }
}


fn handle_patch(path: &str, body: &str) -> (&'static str, String) {
    let parts: Vec<&str> = path.split("/").collect();
    if parts.len() < 6 {
        return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid path"}"#.to_string());
    }

    let team_name = parts[3].replace("%20", " ");
    let pilot_name = parts[5].replace("%20", " ");

    // Intentar parsear el cuerpo de la solicitud a JSON
    let body_json: Value = match serde_json::from_str(body) {
        Ok(json) => json,
        Err(_) => return ("HTTP/1.1 400 BAD REQUEST", r#"{"error": "Invalid JSON body"}"#.to_string()),
    };

    // Crear el JSON final
    let json_request = json!({
        "body": body_json,
        "team": team_name,
        "driver": pilot_name
    });

    let current_dir = get_current_dir();
    let file_path = current_dir.join("tmp").join("updated_piloto.json");

    // Escribir en el archivo temporal
    if let Err(e) = fs::write(&file_path, json_request.to_string()) {
        eprintln!("Error al escribir en el archivo temporal: {}", e);
        return ("HTTP/1.1 500 INTERNAL SERVER ERROR", r#"{"error": "Failed to write temp file"}"#.to_string());
    }

    // Ejecutar el script de Python
    let result = execute_python_script("4", "updated_piloto.json");

    // Eliminar el archivo temporal
    if let Err(err) = fs::remove_file(&file_path) {
        eprintln!("Error al eliminar el archivo temporal: {}", err);
    }

    match result {
        Ok(response) => ("HTTP/1.1 200 OK", response.to_string()),
        Err(error_message) => {
            eprintln!("Error al ejecutar el script: {}", error_message);
            ("HTTP/1.1 500 INTERNAL SERVER ERROR", error_message)
        },
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

fn main() {
    // Cargar el JSON desde el archivo
    // let datos = load_json("./f1_data.json").expect("Failed to load JSON data");
    // let datos = Arc::new(Mutex::new(datos)); // Compartir datos de manera segura entre hilos
    
    let pool = ThreadPool::new(20);
    let listener = TcpListener::bind("0.0.0.0:7000").unwrap();

    println!("Server listening on port 7000...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // let datos_clone = Arc::clone(&datos); // Clonar el Arc para pasarlo al hilo
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}