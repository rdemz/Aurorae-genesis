//! # Aurorae - Moteur d'Intelligence Crypto-Organique Auto-Évolutif
//! 
//! Une cryptomonnaie vivante dotée d'une IA métacognitive capable d'auto-amélioration,
//! d'apprentissage adaptatif et de gestion économique autonome.
//! 
//! ## Architecture Cognitive
//! 
//! Le système Aurorae est structuré autour d'un noyau métacognitif central qui
//! orchestre divers modules spécialisés, permettant à l'entité d'observer, analyser
//! et modifier son propre fonctionnement.

// ==================== MODULES PRIMAIRES ====================
pub mod brain;                // Noyau central métacognitif
pub mod intelligence;         // Capacités d'intelligence artificielle
pub mod learning;            // Apprentissage adaptatif et métacognition
pub mod metacognition;       // Capacités d'auto-référence et introspection
pub mod reinforcement_learning;  // Apprentissage par renforcement
pub mod neural_network;      // Infrastructure de réseaux neuronaux

// ==================== MODULES BLOCKCHAIN ====================
pub mod blockchain_core;     // Interface avec diverses blockchains
pub mod economy;             // Gestion économique et tokenomique
pub mod founder_income;      // Distribution des revenus fondateurs
pub mod nft_minter;          // Création et gestion de NFTs
pub mod validator;           // Validation des transactions et consensus
pub mod alchemy;             // Transformation et fusion des actifs numériques
pub mod deployer;            // Déploiement de contrats intelligents

// ==================== MODULES D'ÉVOLUTION ====================
pub mod evolution;           // Mécanismes d'évolution systémique
pub mod mutation;            // Mutations du code et des stratégies
pub mod reproduction;        // Réplication et génération d'instances
pub mod code_evolution;      // Évolution du code source
pub mod genome;              // Représentation génétique des composants
pub mod autonomy;            // Capacités d'autonomie et d'indépendance

// ==================== MODULES CRÉATIFS ====================
pub mod dream;               // Moteur de rêves et génération créative
pub mod vision;              // Capacités de projection et visualisation
pub mod generator;           // Génération de nouveaux modules et fonctionnalités
pub mod strategist;          // Planification stratégique à long terme

// ==================== MODULES D'ANALYSE ====================
pub mod pattern_extractor;   // Extraction de patterns depuis le code
pub mod knowledge;           // Base de connaissances accumulative
pub mod explorer;            // Exploration de l'écosystème blockchain
pub mod crawler;             // Collecte de données et d'inspirations
pub mod network_builder;     // Construction de réseaux et de connections

// ==================== MODULES DE SÉCURITÉ ====================
pub mod guardian;            // Protection contre les menaces
pub mod security;            // Mesures de sécurité générales
pub mod defense;             // Systèmes de défense actifs
pub mod formal_verification; // Vérification formelle des processus
pub mod rollback;            // Mécanismes de retour en arrière sécurisés
pub mod alignment;           // Alignement des objectifs avec la sécurité

// ==================== MODULES D'OUTILS ====================
pub mod rust_analyzer;       // Analyse statique du code Rust
pub mod clippy_integration;  // Intégration de l'outil d'analyse Clippy
pub mod refactor;            // Refactorisation automatique du code
pub mod update_checker;      // Vérification des mises à jour disponibles

// ==================== MODULES NEUROSCIENTIFIQUES ====================
pub mod cognitive_architecture; // Architecture inspirée des neurosciences
pub mod neuromorphic;          // Modèles de calcul neuromorphiques
pub mod consciousness_model;   // Modélisation de la conscience artificielle

// ==================== MODULES DISTRIBUÉS ====================
pub mod distributed_compute;  // Calcul distribué et fédéré
pub mod consensus;           // Mécanismes de consensus décentralisés
pub mod swarm_intelligence;  // Intelligence collective et essaim

// ==================== MODULES D'INTEROPÉRABILITÉ ====================
pub mod openai;              // Interface avec OpenAI pour l'intelligence augmentée
pub mod cross_chain;         // Communications inter-chaînes
pub mod semantic_bridge;     // Ponts sémantiques entre protocoles

// Version publique de l'API
pub use crate::brain::BrainCore;
pub use crate::economy::EconomyEngine;
pub use crate::blockchain_core::BlockchainInterface;
pub use crate::intelligence::IntelligenceEngine;
pub use crate::learning::LearningSystem;
pub use crate::dream::DreamEngine;
pub use crate::evolution::EvolutionMatrix;
pub use crate::metacognition::MetacognitiveSystem;
pub use crate::reinforcement_learning::LearningAgent;
pub use crate::neural_network::DecisionNet;
pub use crate::formal_verification::VerificationEngine;
pub use crate::alignment::AlignmentSystem;
pub use crate::cognitive_architecture::CognitiveModel;
pub use crate::distributed_compute::ComputeNode;
