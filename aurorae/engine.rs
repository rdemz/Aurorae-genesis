//! engine.rs — Moteur central vivant AURORAE++
//! Gère l’orchestration de tous les modules (économie, rêve, IA, mutation…)

use crate::{
    economy::EconomyEngine,
    dream::DreamEngine,
    alchemy::{AlchemyEngine, TokenKind},
    brain::BrainCore,
    guardian::GuardianSentinel,
    generator::CodeGenerator,
    learning::LearningEngine,
    reproduction::ReproductionEngine,
    mutation::MutationEngine,
};

pub struct AuroraeEngine {
    pub economy: EconomyEngine,
    pub dreamer: DreamEngine,
    pub brain: BrainCore,
    pub guardian: GuardianSentinel,
    pub generator: CodeGenerator,
    pub learner: LearningEngine,
    pub reproducer: ReproductionEngine,
    pub mutator: MutationEngine,
}

impl AuroraeEngine {
    pub fn new() -> Self {
        AuroraeEngine {
            economy: EconomyEngine::new(),
            dreamer: DreamEngine::new(),
            brain: BrainCore::new(),
            guardian: GuardianSentinel::new(),
            generator: CodeGenerator::new(),
            learner: LearningEngine::new(),
            reproducer: ReproductionEngine::new(),
            mutator: MutationEngine::new(),
        }
    }

    pub async fn start_autonomous_cycle(&mut self) {
        println!("[AURORAE++] 🔁 Lancement du cycle autonome");

        // 🔷 1. Économie → Génère token + déploiement
        self.economy.simulate_cycle(750.0).await;

        // 🔷 2. Rêve IA + NFT vivant
        self.dreamer.imagine(
            "Aurora-Swarm",
            "Un essaim de micro-intelligences spécialisées déployables",
            "https://arweave.net/nft1.png"
        );

        // 🔷 3. Génération / mutation / apprentissage
        let context = self.learner.analyze_patterns("github_feed");
        let module = self.generator.generate_from(context);
        let validated = self.guardian.validate_module(&module);

        if validated {
            self.mutator.attempt_mutation(&module);
            self.reproducer.create_instance("dream-fabricator");
        }

        // 🔷 4. Affichages
        self.dreamer.list_dreams();
        self.economy.alchemy.list_tokens();
        self.dreamer.list_nfts();
    }
}
