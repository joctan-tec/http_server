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

### Obtener información sobre escuderías
- **Método**: `POST`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Obtiene información de las escuderías (conductores, nombre, edad y país).

### Obtener información sobre escuderías
- **Método**: `PUT`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Obtiene información de las escuderías (conductores, nombre, edad y país).

### Obtener información sobre escuderías
- **Método**: `DELETE`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Obtiene información de las escuderías (conductores, nombre, edad y país).

### Obtener información sobre escuderías
- **Método**: `PATCH`
- **URL**: `http://127.0.0.1:7000/api/escuderias`
  
> **Descripción**: Obtiene información de las escuderías (conductores, nombre, edad y país).

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
El servidor se basa en una arquitectura concurrente, donde cada conexión entrante es manejada por un hilo independiente. Se utiliza un pool de hilos (thread pool) para reutilizar recursos y mejorar la eficiencia del servidor. Además, el servidor puede gestionar tanto solicitudes JSON como en texto plano. Las operaciones HTTP están bien definidas dentro de las funciones que corresponden a cada método, y los datos se gestionan mediante un sistema sencillo de almacenamiento.

### 2. Implementación de la concurrencia 🚧🚧🚧
Para la concurrencia, el servidor utiliza un **modelo multihilo (threading)**. Cada solicitud de cliente es procesada por un hilo del pool de hilos. Se ha implementado un mecanismo de **sincronización** que garantiza que los recursos compartidos no sufran bloqueos ni condiciones de carrera. Se utilizan primitivos de sincronización como **Mutex** y **Arc** para proteger el acceso concurrente a los datos.

### 3. Manejo de cookies 🚧🚧🚧
Las **cookies** se utilizan para mantener el estado de sesión de los usuarios entre solicitudes. Cada vez que un usuario realiza una operación, se genera o valida una cookie asociada a su sesión. Las cookies contienen un identificador único que es almacenado y validado en el servidor, permitiendo gestionar el estado de forma sencilla y segura.

### 4. Instrucciones para ejecutar y probar el servidor 🚧🚧🚧
Para probar el servidor, puedes utilizar la herramienta **Postman**. A continuación, se detallan algunos comandos para verificar el funcionamiento de los endpoints:
- Para obtener la lista de escuderías:
  ```http
  GET http://127.0.0.1:7000/api/escuderias
  ```
- Para crear una nueva escudería:
  ```http
  POST http://127.0.0.1:7000/api/escuderias
  Content-Type: application/json
  Body: {
    "nombre": "Nueva Escudería",
    "conductor": "Juan Pérez",
    "edad": 30,
    "pais": "México"
  }
  ```
- Para cambiar la información de una escudería:
  ```http
  PUT http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]
  Content-Type: application/json
  Body: {
    "nombre": "Nueva Escudería 2.0",
    "conductor": "Juan Pérez",
    "edad": 30,
    "pais": "México"
  }
  ```
- Para eliminar una escudería:
  ```http
  DELETE http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]
  ```
- Para editar la información del conductor (edad, nombre y/o nacionalidad):
  ```http
  PATCH http://127.0.0.1:7000/api/escuderias/[nombreEscuderia]/pilotos/[nombrePiloto]
  ```


### 6. Estructura del proyecto 🚧🚧🚧
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
| Concurrencia con hilos   | ✅       | Se implementó un manejo concurrente usando hilos. |
| Manejo de cookies        | ✅       | Implementado un sistema básico de gestión de cookies. |
| Pruebas con Postman      | ✅       | Todas las funcionalidades fueron probadas con Postman. |

> **Completitud**: 100%

---

## 🛠️ Tecnologías utilizadas

- **Rust**: Lenguaje de programación utilizado para el desarrollo del servidor.
- **Postman**: Herramienta utilizada para realizar pruebas de las operaciones HTTP.

---

## 🔗 Enlaces de interés

- [Documentación de Rust](https://www.rust-lang.org/)
- [Guía sobre HTTP](https://developer.mozilla.org/es/docs/Web/HTTP)

---
