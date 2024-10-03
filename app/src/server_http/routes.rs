use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::io::{Write, BufReader};
use std::net::TcpStream;
use crate::utils::{parse_request_into_hashmap, clean_string, get_current_dir, print_hashmap};


pub type Handler = Box<dyn Fn(&mut TcpStream, &str) + Send + 'static>;

#[derive(Clone)]
pub struct Router {
    routes: Arc<Mutex<HashMap<String, Handler>>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn list_routes(&self) {
        let routes = self.routes.lock().unwrap(); // Acceder al HashMap con un lock

        println!("Rutas registradas:");
        for route in routes.keys() {
            println!("{}", route);
        }
    }

    pub fn add_route<F>(&mut self, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream, &str) + Send + 'static,
    {
        self.routes
            .lock()
            .unwrap()
            .insert(path.to_string(), Box::new(handler));
    } 
    


    pub fn handle_request(&self, stream: &mut TcpStream, path: &str, method: &str) {
        // Crear la clave combinada de método y ruta
        let key = format!("{} {}", method, path);
    
        // Verificar si la ruta con el método existe
        if let Some(handler) = self.routes.lock().unwrap().get(&key) {
            handler(stream, path); // Ejecutar el handler correspondiente
        } else {
            let _ = writeln!(stream, "HTTP/1.1 404 NOT FOUND\r\n\r\n");
        }
    }
}