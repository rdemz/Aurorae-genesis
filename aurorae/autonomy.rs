//! autonomy.rs — Module d’autonomisation des entités vivantes de AURORAE++

use uuid::Uuid;
use rand::Rng;

use crate::alchemy::{AlchemyEngine, TokenKind};
use crate::economy::EconomyEngine;
use crate::intelligence::IntelligenceCore;
use crate::blockchain_core::BlockchainInterface;
use crate::nft_minter::NFTMinter;

/// 💠 Cœur de l'autonomie systémique d'AURORAE++
pub struct AuroraeCore {
    pub economy: EconomyEngine,
    pub intelligence: IntelligenceCore,
    pub forge: AlchemyEngine,
    pub blockchain: BlockchainInterface,
    pub nft_minter: NFTMinter,
}

impl AuroraeCore {
    /// 🧬 Crée une nouvelle instance autonome avec tous les moteurs connectés
    pub fn new() -> Self {
        Self {
            economy: EconomyEngine::new(),
            intelligence: IntelligenceCore::new(),
            forge: AlchemyEngine::new(),
            blockchain: BlockchainInterface::new(),
            nft_minter: NFTMinter::new(),
        }
    }

    /// 🔗 Simule la création d’une identité blockchain unique pour l’entité
    pub async fn create_blockchain_presence(&self) -> String {
        let chain_id = format!("chain-{}", Uuid::new_v4());
        println!("[AURORAE++] 🔗 Chaîne autonome créée : {chain_id}");
        chain_id
    }

    /// 🧠 Initialise tous les sous-systèmes pour créer un réseau vivant auto-fonctionnel
    pub async fn create_autonomous_network(&mut self) {
        let _chain_id = self.create_blockchain_presence().await;

        self.economy.initialize();
        self.intelligence.initialize();

        let starting_capital = 1_000_000.0;
        self.economy.simulate_cycle(starting_capital).await;

        let _token_id = self
            .forge
            .mint_token("Auroraium", TokenKind::Fungible, 1_000_000, 0.05)
            .await;

        println!("[AURORAE++] ✅ Réseau vivant initialisé avec succès.");
    }

    /// ♻️ Déclenche les cycles d'évolution naturelle du réseau vivant
    pub async fn evolve(&mut self) {
        self.intelligence.improve().await;
        self.economy.innovate();

        let bonus_funding = 1_000.0;
        self.economy.add_funds(bonus_funding);

        println!("[AURORAE++] 🧠⚙️ Système auto-évolué avec succès.");
    }

    /// 🔍 Analyse en direct du niveau cognitif et de l’état économique
    pub fn analyze(&self) {
        let level = self.intelligence.get_intelligence_level();
        println!(
            "[AURORAE++] 📊 Analyse cognitive : niveau actuel {:.2}",
            level
        );

        self.economy.financial_report();
    }

    /// 💭 Simule un flux de pensée actif à partir du moteur cognitif
    pub fn simulate_thoughts(&self) {
        self.intelligence.simulate_thought();
        println!("[AURORAE++] 🧬 Pensées autonomes simulées.");
    }
}
