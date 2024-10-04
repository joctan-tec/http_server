use std::collections::HashMap;
use std::env;
use std::net::TcpStream;
use serde_json;
use std::path::PathBuf;
use std::io::{BufReader, BufRead, Read};
use serde_json::{json, Value};

// Obtener el directorio actual
pub fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    env::current_dir()
}

pub fn print_hashmap(hashmap: &HashMap<String, Value>) {

    match serde_json::to_string_pretty(hashmap) {

        Ok(pretty_json) => println!("{}", pretty_json),

        Err(e) => eprintln!("Error serializing HashMap to JSON: {}", e),

    }

}

// Función para limpiar las cadenas de caracteres especiales
pub fn clean_string(string: String) -> String {
    string.replace("\"", "").replace("\\", "")
}

// Función para parsear la solicitud HTTP en un HashMap

pub fn parse_request_into_hashmap(mut buf_reader: BufReader<&TcpStream>) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    let mut headers = HashMap::new();
    let mut request_line = String::new();

    // Leer la primera línea que contiene el método, ruta y versión
    buf_reader.read_line(&mut request_line).unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() >= 3 {
        result.insert("method".to_string(), json!(clean_string(parts[0].to_string())));
        result.insert("path".to_string(), json!(clean_string(parts[1].to_string())));
        result.insert("version".to_string(), json!(clean_string(parts[2].to_string())));
    }

    // Leer los encabezados línea por línea
    let mut content_length = 0;
    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break; // Fin de los encabezados
        }

        // Separar los encabezados por ": " y agregar al HashMap
        let mut header_parts = line.splitn(2, ": ");
        if let (Some(key), Some(value)) = (header_parts.next(), header_parts.next()) {
            let clean_key = clean_string(key.to_string());
            let clean_value = clean_string(value.to_string());
            headers.insert(clean_key.clone(), json!(clean_value.clone()));

            // Si encontramos el encabezado Content-Length, lo usamos para leer el cuerpo
            if clean_key.to_lowercase() == "content-length" {
                content_length = clean_value.parse::<usize>().unwrap_or(0);
            }
        }
    }

    // Almacenar los encabezados en el HashMap principal como un objeto JSON
    result.insert("headers".to_string(), json!(headers));

    // Leer el cuerpo de la solicitud si existe un Content-Length
    if content_length > 0 {
        let mut body = vec![0; content_length];
        buf_reader.read_exact(&mut body).unwrap(); // Leer exactamente content_length bytes
        let body_str = String::from_utf8(body).unwrap_or_default();

        // Intentar deserializar el cuerpo si es JSON
        let body_value: Result<Value, _> = serde_json::from_str(&body_str);
        match body_value {
            Ok(json_body) => {
                result.insert("body".to_string(), json_body); // Insertar como objeto JSON si es válido
            }
            Err(_) => {
                result.insert("body".to_string(), json!(clean_string(body_str))); // Si no es JSON, almacenar como cadena
            }
        }
    }

    // Extraer cookies si están presentes en los encabezados
    if let Some(cookies) = headers.get("Cookie") {
        result.insert("cookies".to_string(), cookies.clone());
    }

    result
}

// Leer todas las líneas del BufReader y devolverlas como un String
pub fn read_lines_to_string(buf_reader: &mut BufReader<&TcpStream>) -> Result<String, std::io::Error> {
    let mut result = String::new();

    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                result.push_str(&l);
                result.push('\n');
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }

    Ok(result)
}