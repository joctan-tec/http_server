use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use std::error::Error;
use crate::utils::get_current_dir;

// Función para obtener los datos de F1 desde un archivo JSON
pub fn get_f1_data() -> Result<HashMap<String, Value>, Box<dyn Error>> {
    let current_dir = get_current_dir()?;  // Manejo de posibles errores al obtener el directorio actual
    let f1_data_path = current_dir.join("data/f1_data.json");

    // Leer el contenido del archivo
    let f1_data = fs::read_to_string(&f1_data_path)
        .map_err(|e| format!("Error reading f1_data.json: {}", e))?;

    // Parsear el contenido como un HashMap
    let f1_data: HashMap<String, Value> = serde_json::from_str(&f1_data)
        .map_err(|e| format!("Error parsing JSON data: {}", e))?;

    Ok(f1_data)
}

// Función para escribir un HashMap en formato JSON a un archivo
pub fn write_json_to_file(hashmap: &HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let pretty_json = serde_json::to_string_pretty(hashmap)
        .map_err(|e| format!("Error serializing HashMap to JSON: {}", e))?;

    let current_dir = get_current_dir()?;  // Obtener el directorio actual
    let f1_data_path = current_dir.join("data/f1_data.json");

    // Asegurarse de que la carpeta 'data' exista, si no, crearla
    let data_dir = current_dir.join("data");
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Error creating 'data' directory: {}", e))?;
    }

    // Escribir el contenido formateado en el archivo
    fs::write(f1_data_path, pretty_json)
        .map_err(|e| format!("Error writing to f1_data.json: {}", e))?;

    Ok(())
}