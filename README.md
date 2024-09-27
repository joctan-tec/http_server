# http_server
Desarrollo de un servidor HTTP v1.x desde cero en Rust, soportando operaciones GET, PUT, POST, DELETE y UPDATE. Implementa manejo de múltiples solicitudes mediante hilos (threads) y gestiona cookies para administración de sesiones.

## Comandos para correr el server
```bash
git clone https://github.com/joctan-tec/http_server.git
cd http_server/src/
cargo run
```
## Endpoints 
Obtener información sobre escuderías (conductores: nombre, edad y país) 
```
Método: GET
http://127.0.0.1:7000/api/escuderias
```
