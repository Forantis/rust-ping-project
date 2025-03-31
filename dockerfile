FROM rust:slim AS runtime

WORKDIR /app

# Installation des dépendances de build en premier (rarement modifiées)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Création d'un utilisateur non-root pour l'exécution
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Copie des fichiers de configuration Cargo
COPY Cargo.toml Cargo.lock* ./

# Mise en cache des dépendances pour ne pas les recompiler à chaque modification
RUN mkdir src && \
    echo "fn main() {println!(\"Hello, world!\");}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Maintenant, copie du vrai code source (souvent modifié, donc en derniere couche)
COPY src src/

# Modification du code pour écouter sur toutes les interfaces
RUN sed -i 's/127.0.0.1/0.0.0.0/g' src/main.rs

RUN cargo build --release

ENV PING_LISTEN_PORT=7878
EXPOSE 7878

RUN chown -R appuser:appuser /app
USER appuser

CMD ["./target/release/ping-project"]