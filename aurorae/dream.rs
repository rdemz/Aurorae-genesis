//! AURORAE++ - dream.rs
//!
//! Ce module simule l'imagination vivante de l'IA. Il génère des idées de projets ou d'améliorations autonomes
//! en dehors des cycles actifs, et propose des prototypes d'évolution spontanée.

use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DreamVision {
    pub id: Uuid,
    pub imagined_at: String,
    pub title: String,
    pub description: String,
    pub relevance_score: u8,
    pub complexity_estimate: u8,
    pub accepted: bool,
}

#[derive(Default)]
pub struct DreamEngine {
    pub dreams: Vec<DreamVision>,
}

impl DreamEngine {
    pub fn new() -> Self {
        Self { dreams: vec![] }
    }

    pub fn imagine(&mut self, title: &str, description: &str, score: u8, complexity: u8) {
        let vision = DreamVision {
            id: Uuid::new_v4(),
            imagined_at: Utc::now().to_rfc3339(),
            title: title.to_string(),
            description: description.to_string(),
            relevance_score: score,
            complexity_estimate: complexity,
            accepted: false,
        };

        println!("[AURORAE++] 💭 Rêve généré : {} (pertinence: {}, complexité: {})", vision.title, score, complexity);
        self.dreams.push(vision);
    }

    pub fn accept(&mut self, id: Uuid) {
        if let Some(dream) = self.dreams.iter_mut().find(|d| d.id == id) {
            dream.accepted = true;
            println!("[AURORAE++] ✅ Rêve accepté comme prototype à concrétiser : {}", dream.title);
        }
    }

    pub fn show_dreams(&self) {
        println!("[AURORAE++] 🌌 CATALOGUE DES RÊVES :");
        for d in &self.dreams {
            println!("- {} | Pertinence: {} | Complexité: {} | Accepté: {}", d.title, d.relevance_score, d.complexity_estimate, d.accepted);
        }
    }
}
