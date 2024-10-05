// Instituto Tecnológico de Costa Rica 
// Sistemas Operativos 
// Proyecto 1: Http Server
// Estudiantes: Brenda Badilla, Isaac Brenes, Joctan Porras
// Descripción:
//     Desarrollo de un servidor HTTP v1.x desde cero en Rust, soportando operaciones GET, PUT, POST, DELETE y UPDATE. 
//     Implementa manejo de múltiples solicitudes mediante hilos (threads)
//     Gestiona cookies para administración de sesiones.
//
// Repositorio: https://github.com/joctan-tec/http_server

mod utils;
use utils::print_hashmap;
mod json_hashmaps;
use json_hashmaps::f1_data_hashmap::get_f1_data;
mod http_functions;
use http_functions::functions::{post_team, put_team, delete_team, patch_driver};
mod server_http;

use serde_json::Value;
use server_http::server::Server;

use std::collections::HashMap;
use std::{io::Write, net::TcpStream, sync::Arc, sync::RwLock};

use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Cookie {
    value: String,
    created_at: u64, // Timestamp en segundos
}

fn print_cookies(cookies: &RwLock<HashMap<usize, Cookie>>) {
    let cookies_map = cookies.read().unwrap();
    for (id, cookie) in cookies_map.iter() {
        println!("Cookie ID: {}, Value: {}, Created At: {}", id, cookie.value, cookie.created_at);
    }
}

fn main() {
    let data_shared = Arc::new(RwLock::new(get_f1_data().unwrap()));
    let cookies: Arc<RwLock<HashMap<usize, Cookie>>> = Arc::new(RwLock::new(HashMap::new()));
    let cookie_counter = Arc::new(AtomicUsize::new(0)); // Contador para cookies

    let mut server = Server::new(20); // Pool de 20 hilos

    // Ruta para obtener escuderías
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "GET",
        "/api/escuderias",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let data = data_shared_clone.read().unwrap();
            let mut response = String::new();

            // Verificar si hay cookies en el request
            let has_cookies = request.get("cookies").is_some();

            let cookie_value;
            let cookie_id;
            let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            if !has_cookies {
                // Solo generar una nueva cookie si no hay cookies existentes
                cookie_id = cookie_counter_clone.fetch_add(1, Ordering::SeqCst);
                cookie_value = format!("session_{}", cookie_id);

                // Guardar la cookie
                {
                    let mut cookies_map = cookies_clone.write().unwrap();
                    cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });
                }

                // Imprimir cookies
                print_cookies(&cookies_clone);
            } else {
                // Si hay cookies, puedes obtener el valor de la cookie existente (esto depende de cómo lo manejes)
                // Aquí podrías agregar lógica para manejar cookies existentes, si es necesario
                cookie_value = String::from("existing_cookie_value"); // Placeholder
            }

            response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{}",
                cookie_value,
                serde_json::to_string(&*data).unwrap()
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para agregar una nueva escudería
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "POST",
        "/api/escuderias",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
            let cookie_id = cookie_counter_clone.fetch_add(1, Ordering::SeqCst);
            let cookie_value = format!("session_{}", cookie_id);
            let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            // Guardar la cookie
            let mut cookies_map = cookies_clone.write().unwrap();
            cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });

            if let Some(body) = request.get("body") {
                match post_team(body.clone(), &mut data) {
                    Ok(_) => {
                        response = format!(
                            "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"message\": \"Team added\"}}",
                            cookie_value
                        );
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"{}\"}}", cookie_value, e);
                    }
                }
            } else {
                response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"Invalid request body\"}}", cookie_value);
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para actualizar una escudería (PUT)
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "PUT",
        "/api/escuderias/:name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
            let cookie_id = cookie_counter_clone.fetch_add(1, Ordering::SeqCst);
            let cookie_value = format!("session_{}", cookie_id);
            let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            // Guardar la cookie
            let mut cookies_map = cookies_clone.write().unwrap();
            cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });

            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let name = path_parts[3];
            println!("name {}", name);

            if let Some(body) = request.get("body") {
                match put_team(name, body.clone(), &mut data) {
                    Ok(_) => {
                        response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"message\": \"Team updated\"}}",
                            cookie_value
                        );
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"{}\"}}", cookie_value, e);
                    }
                }
            } else {
                response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"Invalid request body\"}}", cookie_value);
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para eliminar una escudería
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "DELETE",
        "/api/escuderias/:name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];
            let mut response = String::new();
            let cookie_id = cookie_counter_clone.fetch_add(1, Ordering::SeqCst);
            let cookie_value = format!("session_{}", cookie_id);
            let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            // Guardar la cookie
            let mut cookies_map = cookies_clone.write().unwrap();
            cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });

            match delete_team(team_name, &mut data) {
                Ok(_) => {
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"message\": \"Team deleted\"}}",
                        cookie_value
                    );
                }
                Err(e) => {
                    response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"{}\"}}", cookie_value, e);
                }
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para actualizar un conductor (PATCH)
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "PATCH",
        "/api/escuderias/:team_name/pilotos/:driver_name",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            let mut data = data_shared_clone.write().unwrap();
            let mut response = String::new();
            let cookie_id = cookie_counter_clone.fetch_add(1, Ordering::SeqCst);
            let cookie_value = format!("session_{}", cookie_id);
            let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

            // Guardar la cookie
            let mut cookies_map = cookies_clone.write().unwrap();
            cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });

            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];
            let driver_name = path_parts[5];
            println!("team {} driver {}", team_name, driver_name);

            if let Some(body) = request.get("body") {
                println!("body {}", body);
                match patch_driver(team_name, driver_name, body.clone(), &mut data) {
                    Ok(_) => {
                        response = format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"message\": \"Driver updated\"}}",
                            cookie_value
                        );
                    }
                    Err(e) => {
                        response = format!("HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"{}\"}}", cookie_value, e);
                    }
                }
            } else {
                response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"Invalid request body\"}}", cookie_value);
            }

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    server.start("0.0.0.0", 7000); // Iniciar el servidor
}