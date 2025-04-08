//! main.rs — Point d’entrée du moteur vivant AURORAE++

mod autonomy;
mod economy;
mod intelligence;
mod alchemy;
mod guardian;
mod blockchain_core;
mod nft_minter;
mod founder_income;
mod validator;
mod mutation;
mod evolution;
mod generator;
mod dream;
mod brain;
mod knowledge;
mod learning;
mod vision;
mod network_builder;
mod reproduction;
mod crawler;
mod explorer;
mod security;
mod deployer;

use autonomy::AutonomyCore;

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Lancement du système vivant…");

    let mut core = AutonomyCore::new();

    // ✅ Création de la présence blockchain
    core.create_autonomous_network().await;

    // 📈 Déploiement d'un cycle évolutif
    core.evolve().await;

    // 🧠 Analyse cognitive & économie
    core.analyze();

    // 💭 Simulation cognitive
    core.simulate_thoughts();

    // 🧪 (optionnel) Ajout d’une extension future ici

    println!("[AURORAE++] ✅ Cycle de vie initial complété.");
}
