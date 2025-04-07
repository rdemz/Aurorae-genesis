//! AURORAE++ - genome.rs
//!
//! Ce module encode l'identité génétique de chaque entité créée : chaînes, tokens, modules, IA-filles...
//! Il attribue une empreinte ADN logicielle unique, traçable, évolutive, et compatible avec la mutation vivante.

use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Genome {
    pub id: Uuid,
    pub birth: String,
    pub ancestry: Vec<Uuid>,
    pub traits: HashMap<String, String>,
    pub mutability: u8,
    pub integrity_score: u8,
}

impl Genome {
    pub fn new(traits: HashMap<String, String>, ancestry: Vec<Uuid>, mutability: u8) -> Self {
        let integrity = 100 - (mutability.min(90)); // plus elle est mutable, moins son intégrité initiale est haute
        Self {
            id: Uuid::new_v4(),
            birth: Utc::now().to_rfc3339(),
            ancestry,
            traits,
            mutability,
            integrity_score: integrity,
        }
    }

    pub fn display(&self) {
        println!("[AURORAE++] 🧬 ADN : {}", self.id);
        println!("- Naissance : {}", self.birth);
        println!("- Ancêtres  : {:?}", self.ancestry);
        println!("- Mutabilité : {}%", self.mutability);
        println!("- Intégrité  : {}%", self.integrity_score);
        println!("- Traits :");
        for (k, v) in &self.traits {
            println!("  • {} = {}", k, v);
        }
    }
}
