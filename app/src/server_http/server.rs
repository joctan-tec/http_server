use crate::utils::parse_request_into_hashmap;
use crate::json_hashmaps::f1_data_hashmap::get_f1_data;
use crate::server_http::thread_pool::ThreadPool;
use crate::server_http::routes::{Router, Handler}; 

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Write};
use std::sync::{Arc, Mutex};

pub struct Server {
    router: Router,
    pool: ThreadPool,
}

impl Server {
    pub fn new(pool_size: usize) -> Self {
        Server {
            router: Router::new(),
            pool: ThreadPool::new(pool_size),
        }
    }

    pub fn add_route<F>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream, &str) + Send + 'static,
    {
        // Combina el método HTTP y la ruta como clave
        let key = format!("{} {}", method, path);
        self.router.add_route(&key, handler);
    }

    pub fn start(&self, host: &str, port: u16) {
        let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
        println!("Server listening on {}:{}", host, port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = self.router.clone();  // Clonamos el router para usar en el hilo
                    self.pool.execute(move || {
                        handle_connection(stream, &router);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: &Router) {
    let mut reader = BufReader::new(&stream);
    let request_parts = parse_request_into_hashmap(reader);

    let tipo = request_parts.get("method").unwrap().as_str().unwrap();
    let ruta = request_parts.get("path").unwrap().as_str().unwrap();

    router.handle_request(&mut stream, ruta, tipo);
}