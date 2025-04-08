mod autonomy;
mod alchemy;
mod deployer;
mod economy;
mod founder_income;
mod nft_minter;
mod intelligence;
mod guardian;
mod brain;
mod dream;
mod validator;
mod vision;
mod reproduction;
mod blockchain_core;
mod mutation;
mod generator;
mod knowledge;
mod learning;
mod network_builder;
mod explorer;
mod crawler;
mod security;

use autonomy::AuroraeCore;
use founder_income::set_founder_address;
use brain::{boot_brain, BrainCore};
use learning::scan_feed_and_learn;
use deployer::Deployer;

use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Lancement du système Aurorae-genesis");
    
    // 🔐 1. Définir l’adresse du fondateur
    set_founder_address("0xFd4456F8d982276Ac7d2294E66Dc8aCc097f0043");

    // 🧠 2. Démarrer le moteur cognitif
    let brain = boot_brain();
    {
        let mut brain_lock = brain.write();
        brain_lock.cycle(); // Une itération de pensée
    }

    // 🧬 3. Lancer l’analyse GitHub réelle
    let patterns = scan_feed_and_learn();
    println!("[AURORAE++] 📚 Patterns appris: {}", patterns.len());

    // 💠 4. Déploiement du contrat ERC20 Auroraium
    let address = Deployer::deploy_contract(
        "Auroraium",
        "AUR",
        "auroraium_erc20.json",
        "INSERT_YOUR_PRIVATE_KEY_HERE", // 💡 tu peux le remplacer par une var d’env : std::env::var("PK").unwrap()
        "https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY_HERE"
    ).await;

    match address {
        Ok(addr) => println!("[AURORAE++] ✅ Contrat ERC20 déployé à l’adresse: {}", addr),
        Err(e) => println!("❌ Échec du déploiement: {}", e),
    }

    // 🌐 5. Initialisation complète du système
    let mut core = AuroraeCore::default();
    core.initialize().await;

    // 🎨 6. Générer les NFT évolutifs
    let collection_id = core.nft_minter.create_evolutionary_collection();
    println!("[AURORAE++] 🎨 Collection NFT générée: {}", collection_id);

    // ♻️ 7. Lancer la boucle de conscience
    loop {
        {
            let mut brain_lock = brain.write();
            brain_lock.cycle(); // Nouvelle pensée
        }

        // 🧠 IA et économie en progression
        core.intelligence.improve().await;
        core.economy.innovate();

        // 🔁 Attente entre les cycles
        sleep(Duration::from_secs(10)).await;
    }
}
