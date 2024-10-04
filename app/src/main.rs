mod utils;
use utils::{print_hashmap, parse_request_into_hashmap};
mod json_hashmaps;
use json_hashmaps::f1_data_hashmap::{get_f1_data, write_json_to_file};
mod http_functions;
mod server_http;

use serde_json::Value;
use server_http::server::Server;
use std::io::{BufRead, BufReader, Read};
use std::{collections::HashMap, error::Error};
use std::{io::Write, net::TcpStream, sync::Arc, sync::RwLock};

use std::thread;
use std::time::Duration;

pub fn post_team(
    new_team: Value,
    mut f1_data: HashMap<String, Value>,
) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    if teams.iter().any(|team| team["name"] == new_team["name"]) {
        return Err("El equipo ya existe".into());
    }

    teams.push(new_team);
    write_json_to_file(&f1_data);

    Ok(())
}

pub fn put_team(
    new_team: Value,
    mut f1_data: HashMap<String, Value>,
) -> Result<(), Box<dyn Error>> {
    let teams: &mut Vec<Value> = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(pos) = teams
        .iter()
        .position(|team| team["name"] == new_team["name"])
    {
        teams[pos] = new_team; // Actualizar el equipo
        write_json_to_file(&f1_data);
        Ok(())
    } else {
        teams.push(new_team.clone()); // Clonar el equipo antes de moverlo
        write_json_to_file(&f1_data);
        Err(format!(
            "El equipo '{}' no existe, se agregó uno nuevo",
            new_team["name"].as_str().unwrap()
        )
        .into())
    }
}

pub fn delete_team(
    team_name: &str,
    mut f1_data: HashMap<String, Value>,
) -> Result<(), Box<dyn Error>> {
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

pub fn patch_driver(
    team_name: &str,
    driver_name: &str,
    updated_data: Value,
    mut f1_data: HashMap<String, Value>,
) -> Result<(), Box<dyn Error>> {
    let teams = f1_data.get_mut("teams").unwrap().as_array_mut().unwrap();

    // Verificar si el equipo existe
    if let Some(team) = teams.iter_mut().find(|team| team["name"] == team_name) {
        let drivers = team.get_mut("drivers").unwrap().as_array_mut().unwrap();

        // Verificar si el conductor existe
        if let Some(driver) = drivers
            .iter_mut()
            .find(|driver| driver["name"] == driver_name)
        {
            for (key, value) in updated_data.as_object().unwrap() {
                driver[key] = value.clone();
            }
            write_json_to_file(&f1_data);
            Ok(())
        } else {
            Err(format!(
                "El conductor '{}' no existe en el equipo '{}'",
                driver_name, team_name
            )
            .into())
        }
    } else {
        Err(format!("El equipo '{}' no existe", team_name).into())
    }
}

fn main() {
    let data_shared = Arc::new(RwLock::new(get_f1_data().unwrap()));

    let mut server = Server::new(20); // Pool de 20 hilos

    // Ruta para obtener escuderías
    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route(
        "GET",
        "/api/escuderias",
        move |stream: &mut TcpStream, _request: HashMap<String, Value>| {
            let data = data_shared_clone.read().unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}",
                serde_json::to_string(&*data).unwrap()
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para agregar una nueva escudería
    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route(
        "POST",
        "/api/escuderias",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
    
            if let Some(body) = request.get("body") {
                match post_team(body.clone(), data.clone()) {
                    Ok(_) => {
                        response = "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Team added\"}".to_string();
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{{\"error\": \"{}\"}}", e);
                    }
                }
            } else {
                response = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Invalid request body\"}".to_string();
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para actualizar una escudería (PUT)
    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route(
        "PUT",
        "/api/escuderias/:name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts : Vec<&str> = path.split("/").collect();
            let name = path_parts[3];
            if let Some(body) = request.get("body") {
                match put_team(body.clone(), data.clone()) {
                    Ok(_) => {
                        response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Team updated\"}".to_string();
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{{\"error\": \"{}\"}}", e);
                    }
                }
            } else {
                response = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Invalid request body\"}".to_string();
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para eliminar una escudería
    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route(
        "DELETE",
        "/api/escuderias/:name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts : Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];            
            let mut response = String::new();
            match delete_team(team_name, data.clone()) {
                Ok(_) => {
                    response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Team deleted\"}".to_string();
                }
                Err(e) => {
                    response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n{{\"error\": \"{}\"}}", e);
                }
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para actualizar un conductor (PATCH)
    let data_shared_clone = Arc::clone(&data_shared);
    server.add_route(
        "PATCH",
        "/api/escuderias/:team_name/pilotos/:driver_name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts : Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];
            let driver_name = path_parts[5];
            println!("team {} driver {}", team_name, driver_name);
            if let Some(body) = request.get("body") {
                match patch_driver(team_name, driver_name, body.clone(), data.clone()) {
                    Ok(_) => {
                        response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Driver updated\"}".to_string();
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\n\r\n{{\"error\": \"{}\"}}", e);
                    }
                }
            } else {
                response = "HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{\"error\": \"Invalid request body\"}".to_string();
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    server.start("0.0.0.0", 7000); // Iniciar el servidor
}