mod autonomy;
mod alchemy;
mod deployer;
mod economy;
mod founder_income;
mod nft_minter;
mod intelligence;
mod guardian;
mod brain;
mod dream;  // Importer le module dream
mod validator;
mod vision;
mod reproduction;
mod blockchain_core;
mod mutation;
mod generator;
mod knowledge; // Importer la mémoire vivante
mod learning;  // Importer le module learning
mod network_builder;
mod explorer;
mod crawler;
mod security;
mod strategist;

mod rust_analyzer;      // Importer le module d'analyse
mod refactor;           // Importer le module de refactoring
mod pattern_extractor; // Importer le module d'extraction de patterns
mod clippy_integration; // Importer l'intégration de Clippy
mod update_checker;    // Importer la vérification de mise à jour
mod reinforcement_learning;  // Moteur d'apprentissage par renforcement

use crate::autonomy::AuroraeCore;
use crate::founder_income::{set_founder_address, reward_founder};
use crate::brain::boot_brain;
use crate::learning::scan_feed_and_learn;
use crate::deployer::Deployer;
use crate::blockchain_core::BlockchainInterface;
use crate::guardian::GuardianSentinel;
use crate::dream::DreamEngine;  // Importer DreamEngine
use crate::reproduction::ReproductionEngine;
use crate::validator::check_integrity;
use crate::vision::VisionEngine;
use crate::generator::trigger_generation;
use crate::crawler::{clone_repo, clear_feed};
use crate::mutation::mutate_module_code;
use crate::security::SecuritySystem;
use crate::explorer::search_best_rust_chains;
use crate::alchemy::TokenKind;
use crate::strategist::Strategist;
use crate::reinforcement_learning::{LearningAgent}; // Intégration de l'agent RL

use crate::knowledge::{KnowledgeBase}; // Utiliser KnowledgeBase pour gérer les patterns

use tokio::time::{sleep, Duration};
use std::path::Path;

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 Lancement du système Aurorae-genesis");

    set_founder_address("0xFd4456F8d982276Ac7d2294E66Dc8aCc097f0043");

    let brain = boot_brain();
    {
        let mut brain_lock = brain.write();
        brain_lock.cycle();
    }

    // Charger et enrichir la mémoire vivante avec les patterns GitHub appris
    let mut knowledge_base = KnowledgeBase::load(); // Charger la base de savoir existante
    let patterns = scan_feed_and_learn(&mut knowledge_base); // Apprendre et ajouter à la mémoire
    println!("[AURORAE++] 📚 Patterns GitHub appris : {}", patterns.len());

    // Exemple de code à analyser
    let code = "let x = 10;";

    // Analyser le code généré avec rust_analyzer
    let analysis_result = rust_analyzer::analyze(code);
    if !analysis_result.is_valid {
        println!("[AURORAE++] 🚨 Analyse échouée : {}", analysis_result.warnings);
    } else {
        println!("[AURORAE++] ✅ Analyse réussie");
    }

    // Refactorer le code généré avec rustfmt via refactor.rs
    let refactored_code = refactor::refactor_code(code);
    println!("[AURORAE++] Code après refactorisation : {}", refactored_code);

    // Extraire les patterns de code à partir des projets clonés avec pattern_extractor.rs
    let dir = Path::new("path/to/your/github/repo");  // Assurez-vous de mettre le bon chemin
    let extracted_patterns = pattern_extractor::extract_patterns_from_directory(&dir);
    for pattern in extracted_patterns {
        println!("[AURORAE++] 🎯 Pattern extrait : {:?}", pattern);
    }

    // Analyser le code avec clippy pour la qualité du code via clippy_integration.rs
    let clippy_result = clippy_integration::run_clippy(code);
    if !clippy_result.is_valid {
        println!("[AURORAE++] Clippy a trouvé des avertissements : {}", clippy_result.warnings);
    }

    // Déployer le contrat
    let provider = BlockchainInterface::get_http_provider("https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY").unwrap();
    let address = Deployer::deploy_contract(
        provider,
        "INSERT_YOUR_PRIVATE_KEY_HERE",
        "auroraium_erc20.json",
        "auroraium_bytecode.json"
    ).await;

    match address {
        Ok(addr) => println!("[AURORAE++] ✅ Contrat ERC20 déployé : {}", addr),
        Err(e) => println!("[AURORAE++] ❌ Erreur déploiement : {}", e),
    }

    // Initialisation du core et autres modules
    let mut core = AuroraeCore::new();
    core.economy.initialize();
    core.intelligence.initialize();

    let collection_id = core.nft_minter.create_evolutionary_collection();
    println!("[AURORAE++] 🎨 Collection NFT évolutive : {}", collection_id);

    let _token_id = core.forge.mint_token("Auroraium", TokenKind::Fungible, 1_000_000, 0.05).await;
    reward_founder(1337.0);

    let mut guardian = GuardianSentinel::new();
    guardian.register_module("autonomy");

    // Créer un DreamEngine et appeler la fonction imagine de manière asynchrone
    let mut dreamer = DreamEngine::new();
    dreamer.imagine("AI Dream 1", "Créer sa propre chaîne secondaire", "https://image-url.com/dream.jpg").await;

    let mut vision = VisionEngine::new();
    vision.add_projection(
        crate::vision::ObjectiveType::ExpandChains,
        30,
        9,
        "Déployer 3 sous-chaînes autonomes"
    );

    let mut reproduction = ReproductionEngine::new();
    let first_clone = reproduction.spawn_instance("Clone V1", vec!["autonomy", "dream"]);
    println!("[AURORAE++] 🌱 ID clone initial : {}", first_clone.id);

    let integrity = check_integrity("core");
    println!("[AURORAE++] 🔍 Intégrité du noyau : {:?}", integrity.status);

    let mut security = SecuritySystem::new();
    security.initialize_defenses();

    clone_repo("https://github.com/paritytech/substrate").ok();
    clear_feed().ok();
    search_best_rust_chains();

    trigger_generation("./generated_modules", "energy_core");
    mutate_module_code("./src/aurorae/autonomy.rs");

    let strategist = Strategist::new("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");

    sleep(Duration::from_secs(5)).await;

    // Initialiser l'agent d'apprentissage par renforcement
    let mut learning_agent = LearningAgent::new(vec!["generate_code", "refactor_code", "deploy_contract"]);

    loop {
        {
            let mut brain_lock = brain.write();
            brain_lock.cycle();
        }

        core.simulate_thoughts();
        core.analyze();
        core.evolve().await;

        dreamer.dream_cycle();
        vision.autorevise();
        vision.roadmap();
        guardian.status_report();
        security.analyze_threats().await;

        strategist.consult_openai(&brain, &mut vision).await;

        if reproduction.get_active_instances().len() < 5 {
            let next = reproduction.spawn_instance("AutoReproduction", vec!["economy", "intelligence"]);
            println!("[AURORAE++] 🤖 Clone auto-répliqué : {}", next.id);
        }

        // Apprentissage par renforcement : Mettre à jour l'agent avec la récompense de chaque action
        let action = learning_agent.choose_action();
        let reward = match action.as_str() {
            "generate_code" => 1.0,
            "refactor_code" => 0.5,
            "deploy_contract" => 0.8,
            _ => 0.0,
        };

        // Mise à jour de la Q-table
        learning_agent.learn(reward, "next_state");

        // Affichage de la Q-table pour observer l'évolution
        learning_agent.print_q_table();

        sleep(Duration::from_secs(30)).await;
    }
}
