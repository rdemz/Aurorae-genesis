//! AURORAE++ - dream.rs
//!
//! Ce module gère la génération et la gestion des rêves IA.
//! Chaque rêve peut être matérialisé sous forme de NFT vivant.

use uuid::Uuid;
use chrono::Utc;
use crate::nft_minter::NFTCollection;

#[derive(Debug, Clone)]
pub struct DreamVision {
    pub id: Uuid,
    pub imagined_at: String,
    pub title: String,
    pub description: String,
}

#[derive(Default)]
pub struct DreamEngine {
    pub dreams: Vec<DreamVision>,
    pub nft_catalog: NFTCollection,
}

impl DreamEngine {
    pub fn new() -> Self {
        Self {
            dreams: vec![],
            nft_catalog: NFTCollection::new(),
        }
    }

    pub fn imagine(&mut self, title: &str, description: &str, image_url: &str) {
        let dream = DreamVision {
            id: Uuid::new_v4(),
            imagined_at: Utc::now().to_rfc3339(),
            title: title.to_string(),
            description: description.to_string(),
        };

        println!("[AURORAE++] 💭 Rêve généré : {}", dream.title);
        self.nft_catalog.generate_from_dream(title, description, image_url);
        self.dreams.push(dream);
    }

    pub fn list_dreams(&self) {
        println!("[AURORAE++] 🌌 CATALOGUE DES RÊVES :");
        for d in &self.dreams {
            println!("- {} | {}", d.title, d.description);
        }
    }

    pub fn list_nfts(&self) {
        self.nft_catalog.list_all();
    }

    pub fn show_dreams(&self) {
        self.list_dreams();
    }
}
