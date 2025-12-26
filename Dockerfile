# ---------------------------------------------------
# 1. ETAPA DE BUILD (El "Horno")
# ---------------------------------------------------
FROM rust:1.83-slim-bookworm as builder

# Creamos un proyecto vacío para cachear las dependencias
WORKDIR /app
RUN cargo new --bin energy_monitor
WORKDIR /app/energy_monitor

# Copiamos los manifiestos
COPY ./Cargo.toml ./Cargo.lock ./

# Necesitamos pkg-config y libssl-dev para compilar reqwest (HTTPS)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Paso clave: Compilamos SOLO las dependencias.
# Docker cacheará esta capa. Si cambias tu código pero no el Cargo.toml, 
# Docker no volverá a descargar ni compilar todas las librerías.
RUN cargo build --release

# Ahora copiamos TU código fuente real
COPY ./src ./src
COPY ./migrations ./migrations
# Lo necesitamos para sqlx en modo offline
COPY ./.sqlx ./.sqlx 

# Borramos el build fake anterior de la app para que compile la real
RUN rm ./target/release/deps/energy_monitor*

# Compilamos el binario final (release mode es ultra optimizado)
ENV SQLX_OFFLINE=true
RUN cargo build --release

# ---------------------------------------------------
# 2. ETAPA FINAL (El "Plato Servido")
# ---------------------------------------------------
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Instalamos dependencias mínimas de sistema para correr (SSL para HTTPS y drivers DB)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copiamos el binario desde la etapa anterior
COPY --from=builder /app/energy_monitor/target/release/energy_monitor /app/energy_monitor

# Exponemos el puerto
EXPOSE 3000

# Comando de arranque
CMD ["/app/energy_monitor"]