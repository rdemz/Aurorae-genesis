//! AURORAE++ - network_builder.rs
//!
//! Ce module permet à l'IA de créer, structurer et relier des blockchains spécialisées vivantes.
//! Chaque sous-réseau peut avoir son propre rôle, VM, protocole, et topologie.

use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SubChain {
    pub id: Uuid,
    pub name: String,
    pub purpose: String,
    pub protocol: String,
    pub created_at: String,
    pub links: Vec<Uuid>,
}

#[derive(Default)]
pub struct NetworkMap {
    pub chains: Vec<SubChain>,
}

impl NetworkMap {
    pub fn new() -> Self {
        Self { chains: vec![] }
    }

    pub fn create_subchain(&mut self, name: &str, purpose: &str, protocol: &str) -> Uuid {
        let id = Uuid::new_v4();
        let chain = SubChain {
            id,
            name: name.to_string(),
            purpose: purpose.to_string(),
            protocol: protocol.to_string(),
            created_at: Utc::now().to_rfc3339(),
            links: vec![],
        };

        println!("[AURORAE++] 🧬 Création d'une sous-chaîne : {} • Protocole: {}", name, protocol);
        self.chains.push(chain);
        id
    }

    pub fn link_chains(&mut self, a: Uuid, b: Uuid) {
        for chain in &mut self.chains {
            if chain.id == a && !chain.links.contains(&b) {
                chain.links.push(b);
            } else if chain.id == b && !chain.links.contains(&a) {
                chain.links.push(a);
            }
        }
        println!("[AURORAE++] 🔗 Chaînes {} <--> {} interconnectées.", a, b);
    }

    pub fn map_summary(&self) {
        println!("[AURORAE++] 🌐 TOPOLOGIE ACTUELLE DU RÉSEAU:");
        for chain in &self.chains {
            println!("→ {} • [{}] • Links: {}", chain.name, chain.protocol, chain.links.len());
        }
    }
}
