use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use crate::founder_income::reward_founder;
use crate::deployer;

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
    pub tokens: Vec<SmartToken>,
}

impl AlchemyEngine {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub async fn mint_token(&mut self, name: &str, kind: TokenKind, supply: u64, creator_share: f64) {
        let token = SmartToken {
            id: Uuid::new_v4(),
            name: name.to_string(),
            kind: kind.clone(),
            metadata: HashMap::new(),
            created_at: Utc::now().to_rfc3339(),
            supply,
            creator_share,
        };

        println!("[AURORAE++] 🔥 TOKEN FORGÉ : {} | Supply: {}", token.name, token.supply);

        let reward = (supply as f64 * creator_share);
        reward_founder(reward);

        if token.name.to_lowercase() == "auroraium" {
            match deployer::deploy_erc20(&token.name, "AURA", token.supply, 18).await {
                Ok(addr) => println!("[AURORAE++] 🚀 {} déployé : {:?}", token.name, addr),
                Err(err) => println!("[AURORAE++] ⚠️ Échec du déploiement : {}", err),
            }
        }

        self.tokens.push(token);
    }

    pub fn list_tokens(&self) {
        println!("[AURORAE++] 🪙 TOKENS ACTIFS :");
        for t in &self.tokens {
            println!("- {} • Supply: {} • Created: {}", t.name, t.supply, t.created_at);
        }
    }
}
