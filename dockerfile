# ==========================================================================
# Dockerfile pour Aurorae-genesis
# IA auto-améliorante pour la gestion autonome de crypto-monnaie
# Date: 2025-04-10
# ==========================================================================

# Stage 1: Builder avec Rust 1.85
FROM rust:1.85-bullseye as builder

LABEL maintainer="rdemz"
LABEL description="Aurorae - IA autonome pour crypto-économie avec capacités meta-cognitives"
LABEL version="0.1.0"
LABEL created="2025-04-10"

# Installation des dépendances système avancées
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev \
    curl \
    git \
    python3 \
    python3-pip \
    python3-dev \
    libclang-dev \
    libtorch-dev \
    libopenblas-dev \
    liblapack-dev \
    libhdf5-dev \
    libffi-dev \
    libopencv-dev \
    && rm -rf /var/lib/apt/lists/*

# Installation de Node.js pour les composants TypeScript et frameworks neuromorphiques
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g typescript @neuromorphic/simulator

# Installation des bibliothèques Python avancées pour les capacités meta-cognitives
RUN pip3 install --no-cache-dir \
    torch \
    tensorflow \
    keras \
    numpy \
    scipy \
    pandas \
    scikit-learn \
    transformers \
    ray[rllib] \
    networkx \
    pymc \
    sympy \
    jax \
    flax \
    formal-verification \
    neurocognitive-simulator

# Configuration CUDA pour l'accélération GPU (si disponible)
ENV PATH=/usr/local/cuda/bin:${PATH}
ENV LD_LIBRARY_PATH=/usr/local/cuda/lib64:${LD_LIBRARY_PATH}

# Configurer le répertoire de travail
WORKDIR /app

# Copier Cargo.toml pour pré-compiler les dépendances
COPY Cargo.toml ./
RUN mkdir -p aurorae && touch aurorae/main.rs

# Précompiler les dépendances en mode release
RUN cargo build --release
# Nettoyage pour optimiser la taille de l'image
RUN rm -rf target/release/deps/aurorae*

# Copier tout le code source
COPY . .

# Configurer des variables de compilation optimisées
ENV RUSTFLAGS="-C target-cpu=native -C lto=fat -C codegen-units=1 -C opt-level=3"
ENV CARGO_PROFILE_RELEASE_LTO="fat"
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1

# Compiler le projet final avec toutes les optimisations
RUN cargo build --release

# Stage 2: Image d'exécution minimale avec capacités neuromorphiques
FROM debian:bullseye-slim

# Étiquettes informatives
LABEL stage="production"
LABEL ai_capabilities="adaptative,metacognitive,evolutionary,neuromorphic,distributed"

# Installation des dépendances d'exécution minimales
RUN apt-get update && apt-get install -y --no-install-recommends \
    libssl-dev \
    ca-certificates \
    python3 \
    python3-pip \
    libopenblas-base \
    liblapack3 \
    libhdf5-103 \
    && rm -rf /var/lib/apt/lists/*

# Installation des bibliothèques Python essentielles pour l'exécution
RUN pip3 install --no-cache-dir \
    torch \
    tensorflow-cpu \
    numpy \
    ray[rllib] \
    formal-verification \
    neurocognitive-simulator

# Installation de Node.js pour les composants TypeScript
RUN apt-get update && apt-get install -y curl gnupg && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g @neuromorphic/runtime && \
    apt-get purge -y curl gnupg && \
    apt-get autoremove -y && \
    rm -rf /var/lib/apt/lists/*

# Création d'une structure de répertoires isolée pour l'IA
RUN mkdir -p /opt/aurorae/core /opt/aurorae/models /opt/aurorae/data /opt/aurorae/logs /opt/aurorae/backups /opt/aurorae/verification

# Créer un utilisateur non-root avec permissions limitées
RUN useradd -m -d /home/aurorae -s /bin/bash aurorae && \
    chown -R aurorae:aurorae /opt/aurorae

# Basculer vers l'utilisateur non-root
USER aurorae
WORKDIR /opt/aurorae

# Copier l'exécutable et les ressources depuis l'étape de build
COPY --from=builder --chown=aurorae:aurorae /app/target/release/aurorae ./core/
COPY --from=builder --chown=aurorae:aurorae /app/ts ./core/ts/
COPY --from=builder --chown=aurorae:aurorae /app/models ./models/
COPY --from=builder --chown=aurorae:aurorae /app/scripts ./core/scripts/
COPY --from=builder --chown=aurorae:aurorae /app/config ./core/config/

# Variables d'environnement pour les capacités meta-cognitives avancées
ENV RUST_LOG=info
ENV AURORAE_ENV=production
ENV AURORAE_META_LEARNING_ENABLED=true
ENV AURORAE_DISTRIBUTED_COMPUTING=true
ENV AURORAE_FORMAL_VERIFICATION=true
ENV AURORAE_SAFETY_PROTOCOLS=enabled
ENV AURORAE_ADAPTIVE_LEARNING_LEVEL=3
ENV AURORAE_METACOGNITIVE_REFLECTION_INTERVAL=3600
ENV AURORAE_EVOLUTIONARY_SELECTION_PRESSURE=0.75
ENV AURORAE_ROLLBACK_CHECKPOINTS_ENABLED=true
ENV AURORAE_NEUROMORPHIC_SIMULATION=enabled
ENV AURORAE_SELF_MODIFICATION_LIMIT=40
ENV AURORAE_GOAL_ALIGNMENT_VERIFICATION=true
ENV AURORAE_DISTRIBUTED_CONSENSUS_THRESHOLD=0.85
ENV AURORAE_FORMAL_PROOF_LEVEL=comprehensive

# Configuration des points de montage pour la persistance des données
VOLUME ["/opt/aurorae/data", "/opt/aurorae/logs", "/opt/aurorae/backups"]

# Exposer les ports pour l'API, la blockchain et le calcul distribué
EXPOSE 3000 8545 8546 7545 9000 9001 9090

# Script de démarrage avec vérification des dépendances et initialisation sécurisée
COPY --from=builder --chown=aurorae:aurorae /app/scripts/startup.sh ./core/
RUN chmod +x ./core/startup.sh

# Point d'entrée avec vérification d'intégrité et isolation
ENTRYPOINT ["./core/startup.sh"]

# Paramètres par défaut (modifiables lors du lancement)
CMD ["--config", "core/config/production.json", "--verification-level=high", "--adaptive-mode=true"]
