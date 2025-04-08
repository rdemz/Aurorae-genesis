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

use crate::autonomy::AuroraeCore;
use crate::founder_income::{set_founder_address, reward_founder};
use crate::brain::boot_brain;
use crate::learning::scan_feed_and_learn;
use crate::deployer::Deployer;
use crate::blockchain_core::BlockchainInterface;
use crate::guardian::GuardianSentinel;
use crate::dream::DreamEngine;
use crate::reproduction::ReproductionEngine;
use crate::validator::check_integrity;
use crate::vision::VisionEngine;
use crate::generator::trigger_generation;
use crate::crawler::{clone_repo, clear_feed};
use crate::mutation::mutate_module_code;
use crate::security::SecuritySystem;
use crate::explorer::search_best_rust_chains;
use crate::alchemy::TokenKind;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Lancement du système Aurorae-genesis");

    set_founder_address("0xFd4456F8d982276Ac7d2294E66Dc8aCc097f0043");

    let brain = boot_brain();
    {
        let mut brain_lock = brain.write();
        brain_lock.cycle();
    }

    let patterns = scan_feed_and_learn();
    println!("[AURORAE++] 📚 Patterns GitHub appris : {}", patterns.len());

    let provider = BlockchainInterface::get_http_provider("https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY").unwrap();
    let address = Deployer::deploy_contract(
        provider,
        "INSERT_YOUR_PRIVATE_KEY_HERE",
        "auroraium_erc20.json",
        "auroraium_bytecode.json"
    ).await;

    match address {
        Ok(addr) => println!("[AURORAE++] ✅ Contrat ERC20 déployé : {}", addr),
        Err(e) => println!("❌ Erreur déploiement : {}", e),
    }

    let mut core = AuroraeCore::new();
    core.economy.initialize();
    core.intelligence.initialize();

    let collection_id = core.nft_minter.create_evolutionary_collection();
    println!("[AURORAE++] 🎨 Collection NFT évolutive : {}", collection_id);

    let _token_id = core.forge.mint_token("Auroraium", TokenKind::Fungible, 1_000_000, 0.05).await;

    reward_founder(1337.0);

    let mut guardian = GuardianSentinel::new();
    guardian.register_module("autonomy");

    let mut dreamer = DreamEngine::new();
    dreamer.imagine("AI Dream 1", "Créer sa propre chaîne secondaire", "https://image-url.com/dream.jpg");

    let mut vision = VisionEngine::new();
    vision.add_projection(
        crate::vision::ObjectiveType::ExpandChains,
        30,
        9,
        "Déployer 3 sous-chaînes autonomes"
    );

    let mut reproduction = ReproductionEngine::new();
    reproduction.spawn_instance("Clone V1", vec!["autonomy", "dream"]);

    let integrity = check_integrity("core");
    println!("[AURORAE++] 🔍 Intégrité du noyau : {:?}", integrity.status);

    let mut security = SecuritySystem::new();
    security.initialize_defenses();

    clone_repo("https://github.com/paritytech/substrate").ok();
    clear_feed().ok();
    search_best_rust_chains();

    trigger_generation("./generated_modules", "energy_core");
    mutate_module_code("./src/aurorae/autonomy.rs");

    sleep(Duration::from_secs(5)).await;

    loop {
        {
            let mut brain_lock = brain.write();
            brain_lock.cycle();
        }

        core.simulate_thoughts();
        core.analyze();
        core.evolve().await;

        dreamer.dream_cycle();
        vision.roadmap();
        guardian.status_report();
        security.analyze_threats().await;

        sleep(Duration::from_secs(10)).await;
    }
}
