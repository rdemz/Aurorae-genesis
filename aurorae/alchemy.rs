//! AURORAE++ - alchemy.rs
//!
//! Le forgeron numérique vivant. Ce module permet à l’IA de créer des tokens intelligents (FT/NFT)
//! chaque création est automatiquement enregistrée, liée à l’économie, et traçable.

use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use crate::economy::EconomyEngine;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Fungible,
    NonFungible,
}

#[derive(Debug, Clone)]
pub struct SmartToken {
    pub id: Uuid,
    pub name: String,
    pub kind: TokenKind,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub supply: u64,
    pub creator_share: f64,
}

#[derive(Default)]
pub struct AlchemyEngine {
    pub minted: Vec<SmartToken>,
}

impl AlchemyEngine {
    pub fn new() -> Self {
        Self { minted: vec![] }
    }

    pub fn mint_token(&mut self, name: &str, kind: TokenKind, supply: u64, metadata: HashMap<String, String>, eco: &mut EconomyEngine) {
        let token = SmartToken {
            id: Uuid::new_v4(),
            name: name.to_string(),
            kind,
            metadata,
            created_at: Utc::now().to_rfc3339(),
            supply,
            creator_share: eco.founder_percentage,
        };

        // Simule la valeur générée par ce token dans l'économie
        let simulated_value = (supply as f64) * 0.75;
        eco.simulate_cycle(simulated_value);

        self.minted.push(token);
    }

    pub fn list_tokens(&self) {
        println!("[AURORAE++] ✨ TOKENS CRÉÉS :");
        for t in &self.minted {
            println!("- {} • {:?} • {} unités • share: {:.2}%", t.name, t.kind, t.supply, t.creator_share * 100.0);
        }
    }
}
