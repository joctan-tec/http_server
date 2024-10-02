use std::collections::HashMap;

pub fn holamundo() -> String {
    "Hola Mundo".to_string()
}

pub fn load_data_from_json () -> HashMap<String, String> {
    let mut data = HashMap::new();
    data.insert("key".to_string(), "value".to_string());
    data
}

pub fn get_data_from_json (key: &str) -> String {
    let data = load_data_from_json();
    match data.get(key) {
        Some(value) => value.to_string(),
        None => "No data found".to_string()
    }
}

pub fn get_all_data_from_json () -> HashMap<String, String> {
    load_data_from_json()
}

pub fn add_data_to_json (key: &str, value: &str) -> String {
    let mut data = load_data_from_json();
    data.insert(key.to_string(), value.to_string());
    "Data added".to_string()
}

pub fn update_data_from_json (key: &str, value: &str) -> String {
    let mut data = load_data_from_json();
    match data.get(key) {
        Some(_) => {
            data.insert(key.to_string(), value.to_string());
            "Data updated".to_string()
        },
        None => "No data found".to_string()
    }
}

pub fn delete_data_from_json (key: &str) -> String {
    let mut data = load_data_from_json();
    match data.get(key) {
        Some(_) => {
            data.remove(key);
            "Data deleted".to_string()
        },
        None => "No data found".to_string()
    }
}



