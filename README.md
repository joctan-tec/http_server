# 🖥️ HTTP Server en Rust

Desarrollo de un servidor HTTP v1.x desde cero en **Rust**, soportando operaciones **GET**, **PUT**, **POST**, **DELETE** y **UPDATE**. Implementa manejo de múltiples solicitudes mediante **hilos (threads)** y gestiona **cookies** para administración de sesiones.

## 🚀 Comandos para correr el servidor

```bash
# Clona el repositorio
git clone https://github.com/joctan-tec/http_server.git

# Accede a la carpeta del proyecto
cd http_server/src/

# Ejecuta el servidor
cargo run
```

## 📌 Endpoints

### Obtener información sobre escuderías
- **Método**: `GET`
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

La documentación del proyecto incluye:

- **Introducción**: Descripción general del servidor y sus funcionalidades.
- **Diseño del servidor**: Explicación de la arquitectura, cómo se gestionan las conexiones concurrentes y cómo se implementan las operaciones HTTP.
- **Concurrencia**: Explicación detallada de cómo se manejan los hilos y las técnicas utilizadas para evitar bloqueos y condiciones de carrera.
- **Manejo de cookies**: Descripción de cómo se gestionan las cookies (creación, almacenamiento y eliminación).
- **Pruebas del servidor**: Instrucciones detalladas sobre cómo ejecutar y probar el servidor utilizando **Postman**.
- **Estructura del proyecto**: Explicación de la estructura de carpetas y archivos.
- **Análisis de resultados**: Un resumen del cumplimiento de los objetivos del proyecto.

---

## 📁 Estructura del Proyecto

```bash
http_server/
│
├── src/                 # Código fuente del servidor
├── tests/               # Pruebas unitarias
├── Cargo.toml           # Archivo de configuración de Rust
└── README.md            # Este archivo
```

---

## 📊 Análisis de Resultados

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
