//! intelligence.rs — Moteur cognitif et graphe de connaissance

use std::collections::HashMap;
use crate::intelligence::knowledge::KnowledgeNode;

pub struct IntelligenceCore {
    pub knowledge_graph: HashMap<String, KnowledgeNode>,
}

impl IntelligenceCore {
    pub fn new() -> Self {
        IntelligenceCore {
            knowledge_graph: HashMap::new(),
        }
    }

    pub fn update_graph(&mut self) {
        let target_ids: Vec<String> = self.knowledge_graph
            .iter()
            .filter(|(_, node)| node.needs_update)
            .map(|(id, _)| id.clone())
            .collect();

        for id in target_ids {
            if let Some(node) = self.knowledge_graph.get_mut(&id) {
                node.recalculate();
            }
        }
    }

    // ✅ Méthodes manquantes
    pub fn initialize(&mut self) {
        println!("[AURORAE++] 🧠 Initialisation du graphe cognitif");
    }

    pub async fn improve(&mut self) {
        println!("[AURORAE++] 🧠 Amélioration cognitive en cours...");
    }

    pub fn get_intelligence_level(&self) -> u32 {
        42
    }

    pub fn simulate_thought(&self) {
        println!("[AURORAE++] 💭 Pensée simulée...");
    }
}

pub mod knowledge {
    #[derive(Clone)]
    pub struct KnowledgeNode {
        pub needs_update: bool,
    }

    impl KnowledgeNode {
        pub fn recalculate(&mut self) {
            println!("[AURORAE++] 🔄 Recalcul du noeud cognitif");
            self.needs_update = false;
        }
    }
}
