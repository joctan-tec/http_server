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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;

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

// Función para manejar las cookies
fn handle_cookie(
    request: &HashMap<String, Value>,
    cookies: &Arc<RwLock<HashMap<usize, Cookie>>>,
    cookie_counter: &Arc<AtomicUsize>,
) -> String {
    let has_cookies = request.get("cookies").and_then(Value::as_str);
    let cookie_value;
    let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    if let Some(existing_cookie) = has_cookies {
        // Si hay una cookie existente en la solicitud, usar esa
        cookie_value = existing_cookie.to_string();
    } else {
        // Generar nueva cookie si no hay ninguna en la solicitud
        let cookie_id = cookie_counter.fetch_add(1, Ordering::SeqCst);
        cookie_value = format!("session_{}", cookie_id);
        {
            let mut cookies_map = cookies.write().unwrap();
            cookies_map.insert(cookie_id, Cookie { value: cookie_value.clone(), created_at });
        }
    }

    cookie_value
}

// Función para limpiar cookies expiradas
fn clean_expired_cookies(cookies: &Arc<RwLock<HashMap<usize, Cookie>>>, max_age: u64) {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let mut cookies_map = cookies.write().unwrap();
    
    cookies_map.retain(|_, cookie| now - cookie.created_at < max_age);
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
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas (60 segundos)
            let data = data_shared_clone.read().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{}",
                cookie_value,
                serde_json::to_string(&*data).unwrap()
            );

            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
    );

    // Ruta para obtener escuderías
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "GET",
        "/api/escuderias_lenta",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas (60 segundos)
            let data = data_shared_clone.read().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);
            thread::sleep(Duration::from_secs(5)); // Simular una operación lenta
            let response = format!(
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
            "/api/escuderias_lenta",
            move |stream: &mut TcpStream, request: HashMap<String, Value>| {
                clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas
                let mut data = data_shared_clone.write().unwrap();
                let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);
                
                thread::sleep(Duration::from_secs(5)); // Simular una operación lenta

                let mut response = String::new();
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

    // Ruta para agregar una nueva escudería
    let data_shared_clone = Arc::clone(&data_shared);
    let cookies_clone = Arc::clone(&cookies);
    let cookie_counter_clone = Arc::clone(&cookie_counter);
    server.add_route(
        "POST",
        "/api/escuderias",
        move |stream: &mut TcpStream, request: HashMap<String, Value>| {
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas
            let mut data = data_shared_clone.write().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);

            let mut response = String::new();
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
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas
            let mut data = data_shared_clone.write().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);

            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let name = path_parts[3];

            let mut response = String::new();
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
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas
            let mut data = data_shared_clone.write().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);

            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];
            
            let mut response = String::new();
            match delete_team(team_name, &mut data) {
                Ok(_) => {
                    response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"message\": \"Team deleted\"}}",
                        cookie_value
                    );
                }
                Err(e) => {
                    response = format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\nSet-Cookie: session={}; Max-Age=60; HttpOnly\r\n\r\n{{\"error\": \"{}\"}}", cookie_value, e);
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
            clean_expired_cookies(&cookies_clone, 60); // Limpiar cookies expiradas
            let mut data = data_shared_clone.write().unwrap();
            let cookie_value = handle_cookie(&request, &cookies_clone, &cookie_counter_clone);

            let pathAux = request.get("path").and_then(Value::as_str).unwrap_or("");
            let path = &pathAux.replace("%20", " ");
            let path_parts: Vec<&str> = path.split("/").collect();
            let team_name = path_parts[3];
            let driver_name = path_parts[5];
            

            let mut response = String::new();
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

    // Hilo para imprimir cookies cada cierto tiempo
    let cookies_clone = Arc::clone(&cookies);
    thread::spawn(move || loop {
        print_cookies(&cookies_clone);
        thread::sleep(Duration::from_secs(10));
    });

    server.start("0.0.0.0", 7000);
}

#[cfg(test)]
mod tests {
    use super::*; // Importar el contenido del archivo principal
    use std::collections::HashMap;
    use serde_json::json;
    use std::sync::{Arc, RwLock};
    use std::sync::atomic::AtomicUsize;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Test para verificar si se genera una nueva cookie cuando no hay cookies en la solicitud
    #[test]
    fn test_generate_new_cookie() {
        let cookies: Arc<RwLock<HashMap<usize, Cookie>>> = Arc::new(RwLock::new(HashMap::new()));
        let cookie_counter = Arc::new(AtomicUsize::new(0));
        let request = HashMap::new(); // Solicitud sin cookies

        let cookie_value = handle_cookie(&request, &cookies, &cookie_counter);

        assert!(cookie_value.starts_with("session_"));
        let cookies_map = cookies.read().unwrap();
        assert_eq!(cookies_map.len(), 1);  // Se ha generado una nueva cookie
    }

    // Test para verificar que una cookie existente no se cambia
    #[test]
    fn test_existing_cookie() {
        let cookies: Arc<RwLock<HashMap<usize, Cookie>>> = Arc::new(RwLock::new(HashMap::new()));
        let cookie_counter = Arc::new(AtomicUsize::new(0));

        let created_at = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        // Insertar una cookie existente
        cookies.write().unwrap().insert(0, Cookie {
            value: String::from("existing_cookie"),
            created_at,
        });

        // Simular una solicitud con cookies
        let mut request = HashMap::new();
        request.insert(String::from("cookies"), json!("existing_cookie"));

        let cookie_value = handle_cookie(&request, &cookies, &cookie_counter);

        // Asegurarse de que la cookie existente no cambia
        assert_eq!(cookie_value, "existing_cookie");
        let cookies_map = cookies.read().unwrap();
        assert_eq!(cookies_map.len(), 1);  // No se ha generado una nueva cookie
    }

    // Test para limpiar cookies expiradas
    #[test]
    fn test_clean_expired_cookies() {
        let cookies: Arc<RwLock<HashMap<usize, Cookie>>> = Arc::new(RwLock::new(HashMap::new()));
        let created_at_old = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() - 120;  // Hace 120 segundos
        let created_at_new = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();  // Ahora

        // Insertar cookies, una vieja y una nueva
        cookies.write().unwrap().insert(0, Cookie {
            value: String::from("old_cookie"),
            created_at: created_at_old,
        });
        cookies.write().unwrap().insert(1, Cookie {
            value: String::from("new_cookie"),
            created_at: created_at_new,
        });

        // Limpiar cookies que tengan más de 60 segundos
        clean_expired_cookies(&cookies, 60);

        let cookies_map = cookies.read().unwrap();
        assert_eq!(cookies_map.len(), 1);  // Solo debería quedar la cookie nueva
        assert!(cookies_map.get(&1).is_some());  // La cookie nueva debería estar presente
    }

    #[test]
    fn test_get() {
        let f1_data = get_f1_data().unwrap();
        assert!(!f1_data.is_empty(), "Los datos de F1 no deberían estar vacíos");
    }

    #[test]
    fn test_crud_operations_in_order() {
        // Inicializar los datos compartidos
        let data_shared = Arc::new(RwLock::new(get_f1_data().unwrap()));

        // 1. Test Post: Añadir un equipo
        {
            let new_team = json!({
                "name": "Example Team",
                "drivers": [
                    {
                        "name": "Nombre Apellido",
                        "age": 23,
                        "nacionality": "British"
                    },
                    {
                        "name": "Nombre Apellido2",
                        "age": 22,
                        "nacionality": "Australian"
                    }
                ]
            });

            let mut data = data_shared.write().unwrap();
            let result = post_team(new_team, &mut data);
            assert!(result.is_ok(), "El equipo debería añadirse correctamente");
        }

        // 2. Test Put: Actualizar el equipo añadido
        {
            let updated_team = json!({
                "name": "Example Team",
                "drivers": [
                    {
                        "name": "Max Verstappen5",
                        "age": 26,
                        "nacionality": "Dutch5"
                    },
                    {
                        "name": "Sergio Perez5",
                        "age": 34,
                        "nacionality": "Mexican5"
                    }
                ]
            });

            let mut data = data_shared.write().unwrap();
            let result = put_team("Example Team", updated_team, &mut data);
            assert!(result.is_ok(), "El equipo debería actualizarse correctamente");
        }

        // 3. Test Patch: Actualizar un conductor específico
        {
            let updated_driver = json!({
                "age": 35,
                "nacionality": "British"
            });

            let mut data = data_shared.write().unwrap();
            let result = patch_driver("Mercedes", "Lewis Hamilton", updated_driver, &mut data);
            assert!(result.is_ok(), "Los datos del conductor deberían actualizarse correctamente");
        }

        // 4. Test Delete: Eliminar el equipo añadido
        {
            let mut data = data_shared.write().unwrap();
            let result = delete_team("Example Team", &mut data);
            assert!(result.is_ok(), "El equipo debería eliminarse correctamente");
        }
    }
}