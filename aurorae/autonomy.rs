//! autonomy.rs — Module d’autonomisation des entités vivantes de AURORAE++

use uuid::Uuid;
use rand::Rng;

use crate::alchemy::{AlchemyEngine, TokenKind};
use crate::economy::EconomyEngine;
use crate::intelligence::IntelligenceCore;
use crate::blockchain_core::BlockchainInterface;
use crate::nft_minter::NFTMinter;

pub struct AutonomyCore {
    pub economy: EconomyEngine,
    pub intelligence: IntelligenceCore,
    pub forge: AlchemyEngine,
    pub blockchain: BlockchainInterface,
    pub nft_minter: NFTMinter,
}

impl AutonomyCore {
    pub fn new() -> Self {
        Self {
            economy: EconomyEngine::new(),
            intelligence: IntelligenceCore::new(),
            forge: AlchemyEngine::new(),
            blockchain: BlockchainInterface::new(),
            nft_minter: NFTMinter::new(),
        }
    }

    pub async fn create_blockchain_presence(&self) -> String {
        let chain_id = format!("chain-{}", Uuid::new_v4());
        println!("[AURORAE++] 🔗 Chaîne créée : {chain_id}");
        chain_id
    }

    pub async fn create_autonomous_network(&mut self) {
        let _chain_id = self.create_blockchain_presence().await;

        self.economy.initialize();
        self.intelligence.initialize();

        let value = 1000000.0;
        self.economy.simulate_cycle(value).await;

        let _token_id = self
            .forge
            .mint_token("Auroraium", TokenKind::Fungible, 1000000, 0.05)
            .await;

        println!("[AURORAE++] ✅ Réseau autonome initialisé.");
    }

    pub async fn evolve(&mut self) {
        self.intelligence.improve().await;
        self.economy.innovate();

        let bonus = 1000.0;
        self.economy.add_funds(bonus);

        println!("[AURORAE++] 📈 Autonomie optimisée avec succès.");
    }

    pub fn analyze(&self) {
        let level = self.intelligence.get_intelligence_level();
        println!(
            "[AURORAE++] 🧠 Intelligence actuelle : niveau {}",
            level
        );

        self.economy.financial_report();
    }

    pub fn simulate_thoughts(&self) {
        self.intelligence.simulate_thought();
    }
}
