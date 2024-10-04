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
        let path = request.get("path").unwrap();
        let method = request.get("method").unwrap();
        let key = clean_string(format!("{} {}", method, path));
        

        

        match self.routes.get(key.as_str()) {
            Some(handler) => {
                handler(stream, request);
            }
            None => {
                let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n");
                println!("Ruta no encontrada");
                stream.write(response.as_bytes()).unwrap();
            }
        }

    }
        
}