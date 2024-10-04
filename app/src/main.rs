mod utils;
mod json_hashmaps;
use json_hashmaps::f1_data_hashmap::{get_f1_data, write_json_to_file};
mod http_functions;
mod server_http;



use serde_json::Value;
use std::{collections::HashMap, error::Error};
use server_http::server::Server;
use std::{io::Write, net::TcpStream, sync::Arc};


pub fn post_team(new_team: Value, mut f1_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    if teams.iter().any(|team| team["name"] == new_team["name"]) {
        return Err("El equipo ya existe".into());
    }

    teams.push(new_team);
    write_json_to_file(&f1_data);

    Ok(())
}

pub fn put_team(new_team: Value, mut f1_data : HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(pos) = teams.iter().position(|team| team["name"] == new_team["name"]) {
        teams[pos] = new_team; // Actualizar el equipo
        write_json_to_file(&f1_data);
        Ok(())
    } else {
        teams.push(new_team.clone()); // Clonar el equipo antes de moverlo
        write_json_to_file(&f1_data);
        Err(format!("El equipo '{}' no existe, se agregó uno nuevo", new_team["name"].as_str().unwrap()).into())
    }
}




pub fn delete_team(team_name: &str, mut f1_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(pos) = teams.iter().position(|team| team["name"] == team_name) {
        teams.remove(pos); // Eliminar el equipo si se encuentra
        write_json_to_file(&f1_data);
        Ok(())
    } else {
        Err(format!("El equipo '{}' no existe", team_name).into()) // Retornar un error si no se encuentra
    }
}


pub fn patch_driver(team_name: &str, driver_name: &str, updated_data: Value, mut f1_data: HashMap<String, Value>) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(team) = teams.iter_mut().find(|team| team["name"] == team_name) {
        let drivers = team.get_mut("drivers").unwrap().as_array_mut().unwrap();

        // Verificar si el conductor existe
        if let Some(driver) = drivers.iter_mut().find(|driver| driver["name"] == driver_name) {
            for (key, value) in updated_data.as_object().unwrap() {
                driver[key] = value.clone();
            }
            write_json_to_file(&f1_data);
            Ok(())
        } else {
            Err(format!("El conductor '{}' no existe en el equipo '{}'", driver_name, team_name).into())
        }
    } else {
        Err(format!("El equipo '{}' no existe", team_name).into())
    }
}


fn main() {
    let data_shared = Arc::new(get_f1_data().unwrap());

    let mut server = Server::new(20); // Crear el servidor con un pool de 20 hilos
    
    // Definir las rutas y métodos
    server.add_route("GET", "/", |stream: &mut TcpStream, _request: &str| {
        std::thread::sleep(std::time::Duration::from_secs(7));
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Success!\"}";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });

    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route("GET", "/escuderias", move |stream: &mut TcpStream, _request: &str| {
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_string() + &data_shared_clone.get("escuderias").unwrap().to_string();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    });
    
    // Recibe un JSON con la información de una escudería y la agrega al HashMap
    

    server.start("0.0.0.0", 7000); // Iniciar el servidor
}