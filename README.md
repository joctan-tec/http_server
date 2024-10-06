# 🖥️ HTTP Server en Rust
Este proyecto se centra en la creación de un servidor HTTP v1.x simple pero robusto utilizando **Rust**. El servidor implementa operaciones HTTP como **GET**, **PUT**, **POST**, **DELETE** y **UPDATE**, y está diseñado para manejar múltiples conexiones simultáneamente utilizando **hilos (threads)**. Adicionalmente, se incluye un sistema de gestión de **cookies** para mantener el estado de sesión del usuario.

## 📑 Índice
1. [Comandos para correr el servidor](##-comandos-para-correr-el-servidor)
2. [Endpoints](##-endpoints)
3. [Descripción del Proyecto](#-descripción-del-proyecto)
4. [Objetivos del Proyecto](#-objetivos-del-proyecto)
5. [Requisitos Técnicos](#-requisitos-técnicos)
6. [Documentación](#-documentación)
   - [Diseño del servidor](#2-diseño-del-servidor)
   - [Implementación de la concurrencia](#3-implementación-de-la-concurrencia)
   - [Manejo de cookies](#4-manejo-de-cookies)
   - [Instrucciones para ejecutar y probar el servidor](#5-instrucciones-para-ejecutar-y-probar-el-servidor)
   - [Estructura del proyecto](#6-estructura-del-proyecto)
   - [Análisis de resultados](#7-análisis-de-resultados)
7. [Tecnologías utilizadas](#-tecnologías-utilizadas)
8. [Enlaces de interés](#-enlaces-de-interés)


## 🚀 Comandos para correr el servidor 🚧🚧🚧

```bash
# Clona el repositorio
git clone https://github.com/joctan-tec/http_server.git

# Accede a la carpeta del proyecto
cd http_server/src/

# Ejecuta el servidor
cargo run
```

## 📌 Endpoints 🚧🚧🚧

### Obtener información sobre escuderías
- **Método**: `GET`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Obtiene información de las escuderías (conductores, nombre, edad y país).

### Agregar una escudería
- **Método**: `POST`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Agrega una nueva escudería con toda su información (conductores, nombre, edad y país).

### Editar información de una escudería
- **Método**: `PUT`
- **URL**: `http://127.0.0.1:7000/api/escuderias/[nombre de la escuderia]`
  
> **Descripción**: Edita la información de una escudería (conductores, nombre, edad y país).

### Eliminar una escudería
- **Método**: `DELETE`
- **URL**: `http://127.0.0.1:7000/api/escuderias/[nombre de la escuderia]`
  
> **Descripción**: Elimina una escudería 

### Edita la información de un conductor 
- **Método**: `PATCH`
- **URL**: `http://127.0.0.1:7000/api/escuderias/[nombre de la escuderia]/pilotos/[nombre del piloto]`
  
> **Descripción**: Edita la información de un conductor (nombre, edad y/o nacionalidad).

---

## 📋 Descripción del Proyecto

Este proyecto implementa un servidor HTTP v1.x funcional desde cero utilizando **Rust**. El servidor soporta las principales operaciones HTTP y es capaz de manejar múltiples solicitudes de manera concurrente mediante hilos. También incluye un sistema básico de gestión de **cookies** para manejar sesiones de usuario.

### 🛠️ Objetivos del Proyecto
1. Desarrollar un servidor HTTP que soporte las operaciones HTTP básicas (**GET**, **PUT**, **POST**, **DELETE**, **UPDATE**).
2. Implementar concurrencia utilizando **hilos** para manejar múltiples solicitudes de clientes de forma eficiente.
3. Mitigar bloqueos y evitar colisiones de datos mediante mecanismos de sincronización adecuados.
4. Gestionar sesiones de usuarios mediante **cookies**.
5. Probar el servidor con herramientas como **Postman**.
6. Documentar el código y proporcionar pruebas unitarias para validar cada funcionalidad.

---

## 📌 Requisitos Técnicos

- **Lenguaje**: Rust
- **Operaciones HTTP**: 
  - `GET`: Recuperar recursos del servidor.
  - `POST`: Enviar datos al servidor.
  - `PUT`: Actualizar o crear recursos.
  - `DELETE`: Eliminar recursos.
  - `UPDATE (PATCH)`: Modificar parcialmente un recurso.
  
- **Concurrencia**: Manejo de múltiples conexiones mediante hilos.
- **Manejo de cookies**: Gestión básica de sesiones de usuario.
- **No se requiere HTTPS**: El proyecto no requiere un protocolo seguro (HTTPS).
- **Herramientas de prueba**: El servidor se probó con **Postman**.
- **Formatos soportados**: Archivos JSON y texto plano.

---

## 📚 Documentación


### 1. Diseño del servidor 🚧🚧🚧
El servidor está basado en una arquitectura *multihilo* que permite gestionar múltiples conexiones concurrentes. El diseño utiliza el modelo *cliente-servidor*, donde el servidor espera conexiones de los clientes y responde a las solicitudes HTTP con los datos apropiados. El servidor es capaz de manejar solicitudes HTTP GET, POST, PUT y DELETE, permitiendo la interacción con los recursos de manera eficiente.

### Arquitectura
El servidor sigue un patrón *multihilo*, en el que cada solicitud de cliente se maneja de manera independiente por un nuevo hilo. Esto garantiza que múltiples clientes puedan interactuar simultáneamente sin bloqueo o retraso significativo.

- *Aceptación de conexiones:* El servidor escucha en un puerto definido y acepta conexiones TCP desde los clientes.
- *Manejo de solicitudes:* Una vez que se acepta la conexión, se crea un nuevo hilo para manejar la solicitud entrante.
- *Enrutamiento:* Dependiendo de la operación HTTP (GET, POST, etc.), el servidor enruta la solicitud al controlador adecuado que gestiona la lógica de la operación. Además valida que la ruta siga el formato necesitado para cada operación. 

### 2. Implementación de la concurrencia 🚧🚧🚧
El servidor utiliza un esquema de *concurrencia basado en hilos*. Cada vez que llega una solicitud, el servidor crea un nuevo hilo para gestionarla, lo que permite que múltiples solicitudes sean procesadas de manera concurrente.

### Gestión de Hilos
- *Hilos individuales por conexión:* Cada conexión que recibe el servidor se asigna a un hilo nuevo, lo que permite el manejo concurrente de múltiples clientes sin bloqueo de la aplicación.
- *Pool de hilos:*  Evitar que la máquina se quede sin hilos, se crea un pool para manejar un numero fijo de hilos.

### Mitigación de Bloqueos y Condiciones de Carrera
Para evitar problemas de concurrencia como bloqueos o condiciones de carrera, se implementan las siguientes técnicas:
- *Bloqueo en recursos compartidos:* Se utiliza un mecanismos de sincronización de RUST llamado RwLock para controlar el acceso al recurso compartido, de esta forma permitimos que múltiples lecturas se hagan concurrentemente. Pero solo una escritura a la vez. 
- *Variables atómicas:* En operaciones críticas que requieren modificación de variables compartidas, se emplean variables atómicas para asegurar que sólo un hilo a la vez pueda realizar modificaciones.


### 3. Manejo de cookies 🚧🚧🚧
Las **cookies** se utilizan para mantener el estado de sesión de los usuarios entre solicitudes. Cada vez que un usuario realiza una operación, se genera o valida una cookie asociada a su sesión. Las cookies contienen un identificador único conformado por el nombre session_[numero] siendo el número correspondiente al número se sesión asignado de manera ascendente, que es almacenado y validado en el cliente.

### Creación de Cookies
- Las cookies se generan mediante una función dedicada que asigna un identificador único a cada sesión de usuario conformado por el nombre session_[numero] siendo el número correspondiente al número se sesión asignado de manera ascendente.
- En el cliente se almacena y validan las cookies. 
- Se asigna una expiración de un día como manejo de cookies. 

### Almacenamiento de Cookies
- Las cookies son almacenadas de manera segura en el lado del cliente.

### Eliminación de Cookies
- Las cookies se eliminan en el cliente estableciendo una fecha de expiración pasada en el encabezado Set-Cookie.
- El servidor también puede invalidar una cookie al borrar la sesión correspondiente en su almacenamiento interno, asegurando que cualquier cookie obsoleta no sea reconocida en futuras solicitudes.


### 4. Instrucciones para ejecutar y probar el servidor 🚧🚧🚧
Para ejecutar el servidor se tienen las siguientes opciones:

```bash
# Clona el repositorio
git clone https://github.com/joctan-tec/http_server.git

# Accede a la carpeta del proyecto
cd http_server/src/

# Ejecuta el servidor
cargo run
```

O bien,

```bash
docker run -p 7000:7000 joctan04/http_server_proyecto1_so:latest
```

Para probar el servidor, se puede utilizar la herramienta **Postman**. A continuación, se detallan algunos comandos para verificar el funcionamiento de los endpoints:
- Para obtener la lista de escuderías:
  ```http
  GET http://127.0.0.1:7000/api/escuderias
  ```
- Para crear una nueva escudería:
  ```http
  POST http://127.0.0.1:7000/api/escuderias
  Content-Type: application/json
  {
      "drivers": [
        {
          "age": 25,
          "nacionality": "Dutch",
          "name": "Max Verstappen"
        },
        {
          "age": 33,
          "nacionality": "Mexican",
          "name": "Sergio Perez"
        }
      ],
      "name": "Red Bull Racing"
    }
           ]
         }
  ```
- Para cambiar la información de una escudería:
  ```http
  PUT http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]
  Content-Type: application/json
  {
      "drivers": [
        {
          "age": 25,
          "nacionality": "Dutch",
          "name": "Max Verstappen"
        },
        {
          "age": 33,
          "nacionality": "Mexican",
          "name": "Sergio Perez"
        }
      ],
      "name": "Red Bull Racing"
    }
  ```
- Para eliminar una escudería:
  ```http
  DELETE http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]
  ```
- Para editar la información del conductor (edad, nombre y/o nacionalidad):
  ```http
  PATCH http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]/pilotos/[nombrePiloto]
  {
     "age": 33,
     "nacionality": "Mexican",
     "name": "Sergio Perez"
  }
  ```

---
### 5. Resultados de pruebas

#### Pruebas unitarias

Para ejecutar las pruebas unitarias se tiene que realizar los siguientes pasos:
```bash
# Ejecutar el servidor para poder ejecutar las pruebas de endpoints
cargo run

# Se debe de abrir una nueva consola para las pruebas
cargo test
```

**Resultados de las pruebas**

![alt text](/images/image-11.png)

##### Método `GET`

**Resultado de cookie**
![alt text](/images/image.png)

**Resultado de respuesta**
![alt text](/images/image-0.png)

##### Método `POST`

**Prueba con body correcto**
![alt text](/images/image-1.png)

**Resultado**
![alt text](/images/image-2.png)

**Prueba con body incorrecto**
![alt text](/images/image-3.png)

##### Método `PUT`

**Prueba con body correcto**
![alt text](/images/image-4.png)

**Prueba con body incorrecto**
![alt text](/images/image-5.png)

##### Método `DELETE`

**Prueba con equipo existente**
![alt text](/images/image-7.png)

**Prueba con equipo inexsistente**
![alt text](/images/image-6.png)

##### Método `PATCH`

**Prueba con datos correctos**
![alt text](/images/image-8.png)

**Prueba con datos incorrectos**
![alt text](/images/image-9.png)
![alt text](/images/image-10.png)


---

### 6. Estructura del proyecto
```bash
http_server
├── app
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── data
│   │   └── f1_data.json
│   ├── src
│   │   ├── http_functions
│   │   │   ├── functions.rs
│   │   │   └── mod.rs
│   │   ├── json_hashmaps
│   │   │   ├── f1_data_hashmap.rs
│   │   │   └── mod.rs
│   │   ├── lib.rs
│   │   ├── main.rs
│   │   ├── server_http
│   │   │   ├── mod.rs
│   │   │   ├── routes.rs
│   │   │   ├── server.rs
│   │   │   └── thread_pool.rs
│   │   └── utils.rs
│   ├── target
│   │   ├── ...
│   ├── tests
│   │   └── tests.rs
│   └── tmp
│       ├── delete_body_dummie.json
│       ├── patch_body_dummie.json
│       ├── post_body_dummie.json
│       └── put_body_dummie.json
├── Dockerfile
├── estructura
├── LICENSE
└── README.md
```

### 7. Análisis de resultados
A continuación, se presenta un análisis del cumplimiento de los objetivos establecidos:

| Objetivo                | Estado  | Descripción breve                                |
|-------------------------|---------|--------------------------------------------------|
| Soporte de operaciones HTTP  | ✅       | Todas las operaciones principales implementadas.  |
| Concurrencia con hilos   | ✅       | Se implementó el manejo de cada solicitud mediante hilos y un pool de hilos manejado por el servidor, cada solicitud se envia a un hilo que hará su debido procesamiento, además, se implementa el bloqueo de la información compartida mediante el uso de un RwLock que permite multiples lecturas concurrentes mientras que no haya una operacion de write obteniendo el bloqueo.|
| Manejo de cookies        | ✅       | Implementado un sistema básico de gestión de cookies, cuando el cliente no envia una cookie, el servidor genera un nuevo identificador de cookie y lo envia en los encabezados, asignando una fecha de expiración. Tambien cuando una cookie ha expirado se borra de la tabla manejada en la aplicación. |
| Pruebas con Postman      | ✅       | Todas los métodos http se probaron con postman. |

> Completitud: 100%

---

## 🛠️ Tecnologías utilizadas

- Rust: Lenguaje de programación utilizado para el desarrollo del servidor.
- Postman: Herramienta utilizada para realizar pruebas de las operaciones HTTP.

---

## 🔗 Enlaces de interés

- [Documentación de Rust](https://www.rust-lang.org/)
- [Guía sobre HTTP](https://developer.mozilla.org/es/docs/Web/HTTP)
- [Multithreading](https://doc.rust-lang.org/beta/book/ch20-00-final-project-a-web-server.html)
- [Http Cookies](https://developer.mozilla.org/es/docs/Web/HTTP/Cookies)
- [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html)

---
