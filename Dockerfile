# Usamos la imagen de rust: https://hub.docker.com/layers/library/rust/1.81.0/images/sha256-c9c623bcf8dd793e818cb5ee959b1eb431ebb39c044456e265de8e9815923cc1?context=explore

FROM rust:1.81.0 AS builder

# Establece el directorio de trabajo
WORKDIR /app

# Copia los archivos de configuración y el código fuente de Rust
COPY ./app/Cargo.toml ./
COPY ./app/src ./src
COPY ./app/data ./data
COPY ./app/tmp ./tmp

# Compila el proyecto de Rust
RUN cargo build --release

# Usa una imagen base de Python
FROM python:3.11-slim

# Establece el directorio de trabajo
WORKDIR /app

# Copia el ejecutable de Rust desde la etapa de construcción
COPY --from=builder /app/target/release/http_server_proyecto1_so ./http_server_proyecto1_so

# Copia el script de Python y el archivo de requisitos
COPY ./app/scripts/json_management.py ./scripts/
COPY ./app/scripts/requirements.txt ./scripts/

# Copia los archivos de configuración y los datos
COPY ./app/data ./data
COPY ./app/tmp ./tmp


# Instala las dependencias de Python
RUN pip install --no-cache-dir -r ./scripts/requirements.txt

# Expone el puerto 7000
EXPOSE 7000

# Comando para ejecutar la aplicación de Rust
CMD ["./http_server_proyecto1_so"]