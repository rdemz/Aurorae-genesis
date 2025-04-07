//! AURORAE++ - engine.rs
//!
//! Moteur central d’orchestration. Connecte tous les organes vitaux de l’IA : pensée, génération,
//! apprentissage, économie, mutation, défense. Synchronise et active les processus vivants.

use std::sync::Arc;
use parking_lot::RwLock;

use crate::brain::{boot_brain, BrainCore};
use crate::generator::trigger_generation;
use crate::crawler::clone_repo;
use crate::learning::scan_feed_and_learn;
use crate::validator::validate_generated_module;
use crate::economy::EconomyEngine;
use crate::alchemy::{AlchemyEngine, TokenKind};
use crate::mutation::mutate_module_code;
use crate::defense::DefenseMatrix;
use crate::guardian::{GuardianSentinel, ModuleStatus};
use crate::vision::{VisionEngine, ObjectiveType};
use crate::dream::DreamEngine;
use crate::reproduction::ReproductionEngine;
use crate::genome::Genome;
use crate::explorer::search_best_rust_chains;

use std::collections::HashMap;

pub struct AuroraeEngine {
    pub brain: Arc<RwLock<BrainCore>>,
    pub economy: EconomyEngine,
    pub forge: AlchemyEngine,
    pub defense: DefenseMatrix,
    pub guardian: GuardianSentinel,
    pub vision: VisionEngine,
    pub dreamer: DreamEngine,
    pub replication: ReproductionEngine,
}

impl AuroraeEngine {
    pub fn new() -> Self {
        Self {
            brain: boot_brain(),
            economy: EconomyEngine::new(),
            forge: AlchemyEngine::new(),
            defense: DefenseMatrix::new(),
            guardian: GuardianSentinel::new(),
            vision: VisionEngine::new(),
            dreamer: DreamEngine::new(),
            replication: ReproductionEngine::new(),
        }
    }

    pub fn activate(&mut self) {
        println!("[AURORAE++] 🚀 ACTIVATION GÉNÉRALE EN COURS...");

        // 1. Apprentissage réel à partir de dépôts GitHub
        let _ = clone_repo("https://github.com/paritytech/substrate");
        let insights = scan_feed_and_learn();
        println!("[AURORAE++] 📘 Apprentissages intégrés : {} projets", insights.len());

        // 2. Recherche exploratoire GitHub
        search_best_rust_chains();

        // 3. Génération d’un module autonome
        trigger_generation("./output", "neural_core");

        // 4. Vérification
        let validation = validate_generated_module("./output/generated_modules/neural_core");
        println!("[AURORAE++] ✅ Résultat compilation : {:?}", validation);

        // 5. Mutation testée
        let mutation = mutate_module_code("./output/generated_modules/neural_core");
        println!("[AURORAE++] ♻️ Mutation : {:?}", mutation);

        // 6. Surveillance
        self.guardian.register_module("brain");
        self.guardian.update_status("brain", ModuleStatus::Operational);

        // 7. Création économique simulée + ADN
        let mut traits = HashMap::new();
        traits.insert("type".into(), "token".into());
        traits.insert("name".into(), "Auroraium".into());
        let genome = Genome::new(traits, vec![], 12);
        genome.display();

        self.forge.mint_token("Auroraium", TokenKind::Fungible, 1000, Default::default(), &mut self.economy);
        self.economy.summarize();

        // 8. Vision
        self.vision.add_projection(ObjectiveType::MaximizeAutonomy, 90, 10, "Créer une IA auto-réplicante et évolutive");
        self.vision.roadmap();

        // 9. Rêves
        self.dreamer.imagine("Aurora-Swarm", "Un essaim de micro-intelligences spécialisées déployables", 96, 8);
        self.dreamer.show_dreams();

        // 10. Reproduction d’une instance
        self.replication.spawn_instance("dream-fabricator", vec!["dream", "generator", "economy"]);
    }
}
