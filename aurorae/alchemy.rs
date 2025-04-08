use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use chrono::Utc;

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

#[derive(Default)] 
pub struct AlchemyEngine {
    pub tokens: HashMap<Uuid, SmartToken>, // Utilisation d'un HashMap pour un accès plus rapide
    pub burned_tokens: HashSet<Uuid>, // Suivi des tokens brûlés
}

impl AlchemyEngine {
    pub fn new() -> Self {
        Self { 
            tokens: HashMap::new(),
            burned_tokens: HashSet::new(),
        }
    }

    // Fonction pour créer un token
    pub async fn mint_token(
        &mut self,
        name: &str,
        kind: TokenKind,
        supply: u64,
        creator_share: f64,
    ) -> Result<(), String> {
        if supply == 0 {
            return Err("Supply ne peut pas être nul".to_string());
        }

        let token = SmartToken {
            id: Uuid::new_v4(),
            name: name.to_string(),
            kind,
            metadata: HashMap::new(),
            created_at: Utc::now().to_rfc3339(),
            supply,
            creator_share,
        };

        self.tokens.insert(token.id, token);
        Ok(())
    }

    // Fonction pour brûler un token
    pub fn burn_token(&mut self, token_id: &Uuid) -> Result<(), String> {
        if let Some(token) = self.tokens.remove(token_id) {
            self.burned_tokens.insert(token.id); // Marquer le token comme brûlé
            Ok(())
        } else {
            Err("Token non trouvé".to_string())
        }
    }

    // Fonction pour récupérer un token par son ID
    pub fn get_token(&self, token_id: &Uuid) -> Option<&SmartToken> {
        self.tokens.get(token_id)
    }

    // Fonction pour récupérer tous les tokens
    pub fn get_all_tokens(&self) -> Vec<&SmartToken> {
        self.tokens.values().collect()
    }

    // Fonction de gouvernance: récupérer la part du créateur
    pub fn get_creator_share(&self, token_id: &Uuid) -> Option<f64> {
        self.tokens.get(token_id).map(|token| token.creator_share)
    }

    pub fn initialize(&mut self) {
        println!("[AURORAE++] 🔮 Initialisation du moteur de forge de tokens");
    }
}
