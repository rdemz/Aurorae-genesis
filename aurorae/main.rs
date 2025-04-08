mod autonomy;
mod autonomy::core;
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

use crate::autonomy::core::AuroraeCore; // Corrected import path
use crate::founder_income::set_founder_address;
use crate::brain::boot_brain;
use crate::learning::scan_feed_and_learn;
use crate::deployer::Deployer;
use crate::blockchain_core::BlockchainInterface;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Lancement du système Aurorae-genesis");

    // 🔐 Adresse fondateur
    set_founder_address("0xFd4456F8d982276Ac7d2294E66Dc8aCc097f0043");

    // 🧠 Initialisation cerveau
    let brain = boot_brain();
    {
        let mut brain_lock = brain.write();
        brain_lock.cycle();
    }

    // 📚 Scan GitHub
    let patterns = scan_feed_and_learn();
    println!("[AURORAE++] 📚 Patterns appris : {}", patterns.len());

    // 🌐 Déploiement ERC20 via ALCHEMY
    let provider = BlockchainInterface::get_http_provider("https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY").unwrap();
    
    // Corrected function call with 4 arguments instead of 5
    let address = Deployer::deploy_contract(
        provider,
        "INSERT_YOUR_PRIVATE_KEY_HERE", // Added private key as the correct second argument
        "auroraium_erc20.json",         // ABI path
        "auroraium_bytecode.json"       // Bytecode path
    ).await;

    match address {
        Ok(addr) => println!("[AURORAE++] ✅ Contrat déployé : {}", addr),
        Err(e) => println!("❌ Erreur déploiement : {}", e),
    }

    // 🌱 Initialisation moteur
    let mut core = AuroraeCore::default();
    core.initialize().await;

    // 🎨 Création NFT
    let collection_id = core.nft_minter.create_evolutionary_collection();
    println!("[AURORAE++] 🎨 Collection NFT : {}", collection_id);

    // ♻️ Boucle vivante
    loop {
        {
            let mut brain_lock = brain.write();
            brain_lock.cycle();
        }

        core.intelligence.improve().await;
        core.economy.innovate();

        sleep(Duration::from_secs(10)).await;
    }
}
