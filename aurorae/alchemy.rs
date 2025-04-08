//! alchemy.rs — Mécanisme de forge des tokens

use uuid::Uuid;
use std::collections::HashMap;

#[derive(Clone)]
pub enum TokenKind {
    Fungible,
    NonFungible,
}

#[derive(Clone)]
pub struct SmartToken {
    pub id: Uuid,
    pub name: String,
    pub kind: TokenKind,
    pub metadata: HashMap<String, String>,
    pub created_at: String,
    pub supply: u64,
    pub creator_share: f64,
}

pub struct AlchemyEngine {
    pub tokens: Vec<SmartToken>,
}

impl AlchemyEngine {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub async fn mint_token(
        &mut self,
        name: &str,
        kind: TokenKind,
        supply: u64,
        creator_share: f64,
    ) {
        let token = SmartToken {
            id: Uuid::new_v4(),
            name: name.to_string(),
            kind,
            metadata: HashMap::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            supply,
            creator_share,
        };

        self.tokens.push(token);
    }

    pub fn list_tokens(&self) {
        for t in &self.tokens {
            println!("- {} • {}", t.name, t.supply);
        }
    }
}
