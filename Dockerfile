
# Usamos la imagen de rust: https://hub.docker.com/layers/library/rust/1.81.0/images/sha256-c9c623bcf8dd793e818cb5ee959b1eb431ebb39c044456e265de8e9815923cc1?context=explore

from rust:1.81.0

# Instalamos python y pip

RUN apt-get update && apt-get install -y python3 python3-pip

# Establecemos el directorio de trabajo

WORKDIR /app

# Copiamos el contenido de la carpeta actual al directorio de trabajo

COPY . .

# 
