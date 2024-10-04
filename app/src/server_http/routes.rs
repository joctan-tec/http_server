use core::hash;
use std::sync::Arc;
use std::collections::HashMap;
use std::io::{Write};
use std::net::TcpStream;

use serde_json::Value;

use crate::utils::{print_hashmap, clean_string};

pub type Handler = Arc<Box<dyn Fn(&mut TcpStream, HashMap<String, Value>) + Send + Sync>>;

#[derive(Clone)]
pub struct Router {
    routes: HashMap<String, Handler>, // No es necesario bloquear aquí
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn list_routes(&self) {
        println!("Rutas registradas:");
        for route in self.routes.keys() {
            println!("{}", route);
        }
    }

    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream,HashMap<String, Value>) + Send + Sync + 'static,
    {
        println!("Agregando ruta: {}", path);
        self.routes.insert(path.to_string(), Arc::new(Box::new(handler)));
    }

    pub fn handle_request(&self, request: HashMap<String, Value>, stream: &mut TcpStream) {
        let path = request.get("path").unwrap().as_str().unwrap();
        let method = request.get("method").unwrap().as_str().unwrap();
        let key = clean_string(format!("{} {}", method, path));
    
        if let Some(handler) = self.routes.get(key.as_str()) {
            handler(stream, request);
        } else {
            // Verificar rutas con parámetros
            for (route_key, handler) in &self.routes {
                if let Some(captures) = self.match_route(route_key, &key) {
                    let mut request_with_params = request.clone();
                    
                    // Convert HashMap to serde_json::Map
                    let params_map: serde_json::Map<String, Value> = captures.into_iter().collect();
                    
                    request_with_params.insert("params".to_string(), Value::Object(params_map));
                    handler(stream, request_with_params);
                    return;
                }
            }
    
            let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
            println!("Ruta no encontrada");
            stream.write(response.as_bytes()).unwrap();
        }
    }
    
    
    pub fn match_route(&self, route_key: &str, path: &str) -> Option<HashMap<String, Value>> {        let route_parts: Vec<&str> = route_key.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();
    
        if route_parts.len() != path_parts.len() {
            return None;
        }
    
        let mut params = HashMap::new();
    
        for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
            if route_part.starts_with(':') {
                // Capture the parameter
                let param_name = &route_part[1..]; // remove ':'
                params.insert(param_name.to_string(), Value::String(path_part.to_string()));
            } else if *route_part != *path_part {
                return None; // No match
            }
        }
    
        Some(params)
    }
}