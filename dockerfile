# =====================================================
# Dockerfile pour Aurorae-genesis
# IA auto-améliorante pour la gestion autonome de crypto-monnaie
# =====================================================

# Utilisation d'une image Rust 1.75.0 ou plus récente comme base
FROM rust:1.75-bullseye as builder

# Installation des dépendances système nécessaires
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    curl \
    git \
    python3 \
    python3-pip \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Installation de Node.js pour les composants TypeScript
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g typescript

# Installation des bibliothèques Python nécessaires pour libtorch et autres dépendances
RUN pip3 install --no-cache-dir torch numpy scikit-learn pandas

# Configurer le répertoire de travail
WORKDIR /app

# Copier Cargo.toml et Cargo.lock (s'il existe) pour pré-compiler les dépendances
COPY Cargo.toml ./
RUN mkdir -p aurorae && touch aurorae/main.rs

# Précompiler les dépendances (cette étape est en cache tant que Cargo.toml ne change pas)
RUN cargo build --release

# Copier tout le code source
COPY . .

# Recompiler avec le vrai code source
RUN cargo build --release

# Deuxième étape pour une image plus légère
FROM debian:bullseye-slim

# Installation des dépendances d'exécution minimales
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    python3 \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Installation des bibliothèques Python pour l'exécution
RUN pip3 install --no-cache-dir torch numpy scikit-learn pandas

# Installation de Node.js pour l'exécution des composants TypeScript
RUN apt-get update && apt-get install -y curl && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Créer un utilisateur non-root pour plus de sécurité
RUN useradd -m aurorae
USER aurorae

# Configurer le répertoire de travail
WORKDIR /home/aurorae/app

# Copier l'exécutable compilé depuis l'étape de build précédente
COPY --from=builder --chown=aurorae:aurorae /app/target/release/aurorae ./
# Copier les fichiers TypeScript et autres ressources nécessaires
COPY --from=builder --chown=aurorae:aurorae /app/ts ./ts
# Copier les modèles et ressources d'IA
COPY --from=builder --chown=aurorae:aurorae /app/models ./models
# Copier les scripts et configurations
COPY --from=builder --chown=aurorae:aurorae /app/scripts ./scripts
COPY --from=builder --chown=aurorae:aurorae /app/config ./config

# Variables d'environnement pour configurer le comportement de l'IA
ENV RUST_LOG=info
ENV AURORAE_ENV=production
ENV AURORAE_META_LEARNING_ENABLED=true
ENV AURORAE_DISTRIBUTED_COMPUTING=true
ENV AURORAE_FORMAL_VERIFICATION=true
ENV AURORAE_SAFETY_PROTOCOLS=enabled

# Exposer les ports nécessaires pour l'API et la blockchain
EXPOSE 3000 8545 8546

# Point d'entrée pour lancer l'application
ENTRYPOINT ["./aurorae"]
# Arguments par défaut (peuvent être remplacés lors du lancement du conteneur)
CMD ["--config", "config/production.json"]
