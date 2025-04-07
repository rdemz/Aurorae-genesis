//! AURORAE++ - nft_minter.rs
//!
//! Ce module permet de générer, cataloguer et préparer le mint de NFT vivants créés par l’IA
//! Chaque NFT peut représenter : un rêve, un module, une IA-fille ou une chaîne.

use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AuroraNFT {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub created_at: String,
    pub traits: HashMap<String, String>,
    pub minted: bool,
}

#[derive(Default)]
pub struct NFTCollection {
    pub items: Vec<AuroraNFT>,
}

impl NFTCollection {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn generate_from_dream(&mut self, dream_title: &str, description: &str, image_url: &str) {
        let nft = AuroraNFT {
            id: Uuid::new_v4(),
            title: dream_title.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            created_at: Utc::now().to_rfc3339(),
            traits: HashMap::new(),
            minted: false,
        };

        println!("[AURORAE++] 🎨 NFT créé depuis un rêve : {}", dream_title);
        self.items.push(nft);
    }

    pub fn list_all(&self) {
        println!("[AURORAE++] 📚 NFT VIVANTS ACTUELS :");
        for nft in &self.items {
            println!("- {} • {} • Minted: {}", nft.title, nft.description, nft.minted);
        }
    }

    pub fn mark_as_minted(&mut self, id: Uuid) {
        if let Some(nft) = self.items.iter_mut().find(|n| n.id == id) {
            nft.minted = true;
            println!("[AURORAE++] ✅ NFT marqué comme minté : {}", nft.title);
        }
    }
}
