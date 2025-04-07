//! AURORAE++ - defense.rs
//!
//! Ce module forme le système immunitaire de l'IA. Il détecte, isole, et corrige les anomalies logiques,
//! les erreurs critiques, ou les comportements non alignés avec sa logique vitale.

use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ThreatType {
    CompileFailure,
    AnomalousMutation,
    UnauthorizedModule,
    CorruptedMemory,
    LogicDivergence,
}

#[derive(Debug, Clone)]
pub struct ThreatReport {
    pub id: Uuid,
    pub detected_at: String,
    pub threat_type: ThreatType,
    pub details: String,
    pub neutralized: bool,
}

#[derive(Default)]
pub struct DefenseMatrix {
    pub reports: Vec<ThreatReport>,
}

impl DefenseMatrix {
    pub fn new() -> Self {
        Self { reports: vec![] }
    }

    pub fn detect_threat(&mut self, threat_type: ThreatType, details: &str) {
        let cloned_type = threat_type.clone();
        let report = ThreatReport {
            id: Uuid::new_v4(),
            detected_at: Utc::now().to_rfc3339(),
            threat_type,
            details: details.to_string(),
            neutralized: false,
        };

        println!("[AURORAE++] ⚠️ MENACE DÉTECTÉE : {:?} — {}", cloned_type, details);
        self.reports.push(report);
    }

    pub fn neutralize_latest(&mut self) {
        if let Some(last) = self.reports.last_mut() {
            if !last.neutralized {
                last.neutralized = true;
                println!("[AURORAE++] ✅ MENACE NEUTRALISÉE : {:?}", last.threat_type);
            }
        }
    }

    pub fn list_threats(&self) {
        println!("[AURORAE++] 🛡️ RÉCAPITULATIF DES MENACES :");
        for r in &self.reports {
            println!("- [{}] {:?} | Neutralisé: {} | {}", r.id, r.threat_type, r.neutralized, r.details);
        }
    }
}
