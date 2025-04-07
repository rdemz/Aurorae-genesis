//! AURORAE++ - main.rs

mod deployer;
mod nft_minter;
mod economy;
mod alchemy;
mod dream;
mod founder_income;

use economy::EconomyEngine;
use dream::DreamEngine;
use alchemy::{AlchemyEngine, TokenKind};

#[tokio::main]
async fn main() {
    println!("[AURORAE++] 🚀 SYSTÈME VIVANT INITIALISÉ");

    let mut economy = EconomyEngine::new();
    let mut dreamer = DreamEngine::new();

    // Simulation d'un cycle économique (token + déploiement ERC20)
    economy.simulate_cycle(750.0).await;

    // Génération d'un rêve (avec NFT associé)
    dreamer.imagine(
        "Aurora-Swarm",
        "Un essaim de micro-intelligences spécialisées déployables",
        "https://arweave.net/nft1.png"
    );

    // Affichage des rêves et NFT vivants
    dreamer.show_dreams();
    dreamer.list_nfts();

    // Affichage des tokens générés et leur état
    economy.alchemy.list_tokens();

    println!("[AURORAE++] ✅ CYCLE COMPLET");
}
