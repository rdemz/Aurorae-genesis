//! AURORAE++ - vision.rs
//!
//! Module de projection évolutive. Permet à l'IA d'émettre des visions stratégiques de son propre futur,
//! d'établir des roadmaps vivantes et de guider ses mutations selon des objectifs vitaux.

use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ObjectiveType {
    ImproveLearning,
    OptimizeEconomy,
    ExpandChains,
    RefactorSelf,
    BuildEcosystem,
    SeekKnowledge,
    MaximizeAutonomy,
}

#[derive(Debug, Clone)]
pub struct FutureProjection {
    pub id: Uuid,
    pub created_at: String,
    pub target: ObjectiveType,
    pub horizon_days: u32,
    pub priority: u8,
    pub rationale: String,
}

#[derive(Default)]
pub struct VisionEngine {
    pub projections: Vec<FutureProjection>,
}

impl VisionEngine {
    pub fn new() -> Self {
        Self { projections: vec![] }
    }

    pub fn add_projection(&mut self, target: ObjectiveType, horizon_days: u32, priority: u8, rationale: &str) {
        let proj = FutureProjection {
            id: Uuid::new_v4(),
            created_at: Utc::now().to_rfc3339(),
            target,
            horizon_days,
            priority,
            rationale: rationale.to_string(),
        };

        println!(
            "[AURORAE++] 🧠 Vision projetée : {:?} ({} jours) • Priorité {} → {}",
            proj.target, proj.horizon_days, proj.priority, proj.rationale
        );

        self.projections.push(proj);
    }

    pub fn roadmap(&self) {
        println!("[AURORAE++] 📍 ROADMAP STRATÉGIQUE EN COURS :");
        for proj in &self.projections {
            println!(
                "- {:?} • Horizon: {}j • Priorité: {} • [{}]",
                proj.target, proj.horizon_days, proj.priority, proj.rationale
            );
        }
    }

    /// 🔁 Révision automatique de la vision à chaque cycle.
    pub fn autorevise(&mut self) {
        for proj in &mut self.projections {
            if proj.horizon_days > 0 {
                proj.horizon_days -= 1;
                proj.priority = (proj.priority + 1).min(10);
            }
        }

        let before = self.projections.len();
        self.projections.retain(|p| p.horizon_days > 0);
        let after = self.projections.len();

        if before != after {
            println!(
                "[AURORAE++] 🔄 Révision des visions : {} expirées, {} restantes.",
                before - after,
                after
            );
        }
    }
}
