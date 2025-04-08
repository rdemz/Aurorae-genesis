use aurorae::{
    AuroraeCore, Strategist, VisionEngine, BrainCore, ReproductionEngine, Deployer, BlockchainInterface,
    Thought, Intent, check_integrity, trigger_generation, mutate_module_code, DreamEngine, GuardianSentinel,
};
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role};
use async_openai::Client;
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;
use chrono::Utc;
use rand::Rng;

#[tokio::test]
async fn test_system_integration() {
    // 1. Initialisation du core Aurorae
    let mut core = AuroraeCore::new();
    core.economy.initialize();
    core.intelligence.initialize();

    // 2. Vérifier la création de la collection NFT évolutive
    let collection_id = core.nft_minter.create_evolutionary_collection();
    assert!(collection_id > 0, "Échec de la création de la collection NFT");

    // 3. Vérifier le déploiement d'un contrat ERC20
    let provider = BlockchainInterface::get_http_provider("https://eth-sepolia.g.alchemy.com/v2/YOUR_KEY").unwrap();
    let address = Deployer::deploy_contract(
        provider,
        "INSERT_YOUR_PRIVATE_KEY_HERE",
        "auroraium_erc20.json",
        "auroraium_bytecode.json"
    ).await;
    assert!(address.is_ok(), "Erreur de déploiement du contrat ERC20");

    // 4. Créer un clone (instance d'Aurorae)
    let mut reproduction = ReproductionEngine::new();
    let first_clone = reproduction.spawn_instance("Clone V1", vec!["autonomy", "dream"]);
    assert!(first_clone.id != Uuid::nil(), "Échec de la création d'un clone");

    // 5. Créer un VisionEngine et ajouter une projection
    let mut vision = VisionEngine::new();
    vision.add_projection(
        crate::vision::ObjectiveType::ExpandChains,
        30,
        9,
        "Déployer 3 sous-chaînes autonomes"
    );
    assert!(vision.projections.len() > 0, "Échec de l'ajout d'une projection à la roadmap");

    // 6. Test de l'intégration d'OpenAI via le Stratégiste
    let strategist = Strategist::new("sk-YOUR-OPENAI-KEY");
    let brain = Arc::new(RwLock::new(BrainCore::new()));

    // Ajouter une projection à la vision avant d'effectuer la consultation avec OpenAI
    strategist.consult_openai(&brain, &mut vision).await;
    
    // Vérification de la réponse générée par OpenAI
    let brain_lock = brain.read();
    assert!(brain_lock.cortex.len() > 0, "Aucune pensée n'a été générée après la consultation d'OpenAI");

    // 7. Test de la génération d'un module
    let result = trigger_generation("./generated_modules", "energy_core");
    assert!(result.is_ok(), "Échec de la génération du module");

    // 8. Test de la mutation du module
    let mutation_result = mutate_module_code("./src/aurorae/autonomy.rs");
    assert!(mutation_result.is_ok(), "Échec de la mutation du module");

    // 9. Test de l'intégrité du système
    let integrity = check_integrity("core");
    assert_eq!(integrity.status, "OK", "L'intégrité du système est corrompue");

    // 10. Vérification du clonage automatique (si la population est < 5)
    if reproduction.get_active_instances().len() < 5 {
        let next = reproduction.spawn_instance("AutoReproduction", vec!["economy", "intelligence"]);
        assert!(next.id != Uuid::nil(), "Échec du clonage automatique");
    }

    // 11. Test de la gestion des rêves (DreamEngine)
    let mut dreamer = DreamEngine::new();
    dreamer.imagine("AI Dream 1", "Créer sa propre chaîne secondaire", "https://image-url.com/dream.jpg");
    dreamer.dream_cycle();
    assert!(dreamer.get_dream_count() > 0, "Aucun rêve généré");

    // 12. Test de la surveillance et gestion de module via GuardianSentinel
    let mut guardian = GuardianSentinel::new();
    guardian.register_module("autonomy");
    guardian.status_report();
    assert!(guardian.modules_evolved > 0, "Aucun module n'a évolué");

    // 13. Vérification des clones actifs dans ReproductionEngine
    let active_instances = reproduction.get_active_instances();
    assert!(active_instances.len() > 0, "Aucune instance active");
    
    // 14. Test de la création d'une sous-chaîne dans NetworkBuilder
    let mut network_map = crate::network_builder::NetworkMap::new();
    let subchain_id = network_map.create_subchain("SubChain 1", "Purpose", "Protocol");
    assert!(subchain_id != Uuid::nil(), "Échec de la création de la sous-chaîne");
    
    // 15. Vérification des modules générés dans NetworkBuilder
    network_map.map_summary();
    assert!(network_map.chains.len() > 0, "Aucune chaîne dans le réseau");

    // 16. Test de la sécurité via SecuritySystem
    let mut security = crate::security::SecuritySystem::new();
    security.analyze_threats().await;
    assert!(security.get_security_level() > 0.0, "Sécurité insuffisante");
}

