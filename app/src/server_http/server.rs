use serde_json::Value;

use crate::utils::{parse_request_into_hashmap, print_hashmap};
use crate::json_hashmaps::f1_data_hashmap::get_f1_data;
use crate::server_http::thread_pool::ThreadPool;
use crate::server_http::routes::{Router, Handler}; 

use std::collections::HashMap;
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

    pub fn add_route<F>(&mut self,method: &str, path: &str, handler: F)
    where
        F: Fn(&mut TcpStream,HashMap<String, Value>) + Send + Sync + 'static,
    {
        let pair = format!("{} {}", method, path);
        self.router.add_route(&pair, handler);
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
    

    // Enviar el request parseado al router
    router.handle_request(request_parts, &mut stream);
}