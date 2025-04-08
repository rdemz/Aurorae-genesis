//! intelligence.rs — Moteur cognitif et analyse du graphe de connaissance

use std::collections::HashMap;
use crate::knowledge::KnowledgeNode;

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
        // 🧠 Collecte des noeuds à modifier pour éviter les emprunts croisés
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
}

// Exemple de définition du noeud cognitif
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
