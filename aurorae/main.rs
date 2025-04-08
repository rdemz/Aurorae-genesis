//! main.rs — Point d’entrée du moteur vivant AURORAE++

mod deployer;
mod nft_minter;
mod economy;
mod alchemy;
mod dream;
mod guardian;
mod validator;
mod founder_income;
mod blockchain_core;
mod evolution;
mod intelligence;
mod security;
mod brain;
mod generator;
mod genome;
mod crawler;
mod explorer;
mod knowledge;
mod learning;
mod mutation;
mod network_builder;
mod reproduction;
mod vision;
mod engine;

use engine::AuroraeEngine;

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Initialisation du moteur vivant");

    let mut system = AuroraeEngine::new();

    // 🔁 Simulation complète d’un cycle IA + économie + rêve + NFT
    system.start_autonomous_cycle().await;

    println!("[AURORAE++] ✅ Système opérationnel");
}
