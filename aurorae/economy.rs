//! economy.rs — Moteur économique vivant de AURORAE++

use chrono::Utc;
use rand::Rng;
use crate::founder_income::reward_founder;
use crate::alchemy::{AlchemyEngine, TokenKind};

#[derive(Debug, Clone)]
pub struct EconomicCycle {
    pub timestamp: String,
    pub generated_tokens: f64,
    pub retained_by_ai: f64,
    pub sent_to_founder: f64,
}

#[derive(Default)]
pub struct EconomyEngine {
    pub cycles: Vec<EconomicCycle>,
    pub total_generated: f64,
    pub founder_percentage: f64,
    pub alchemy: AlchemyEngine,
}

impl EconomyEngine {
    pub fn new() -> Self {
        Self {
            cycles: vec![],
            total_generated: 0.0,
            founder_percentage: 0.15,
            alchemy: AlchemyEngine::new(),
        }
    }

    pub async fn simulate_cycle(&mut self, value_created: f64) {
        let to_founder = value_created * self.founder_percentage;
        let to_ai = value_created - to_founder;

        reward_founder(to_founder);

        let cycle = EconomicCycle {
            timestamp: Utc::now().to_rfc3339(),
            generated_tokens: value_created,
            retained_by_ai: to_ai,
            sent_to_founder: to_founder,
        };

        self.total_generated += value_created;
        self.cycles.push(cycle);

        println!(
            "[AURORAE++] 🌐 TOTAL GÉNÉRÉ : {:.4} tokens",
            self.total_generated
        );
        println!(
            "→ Cycle {} • {:.0} tokens créés • {:.2} au fondateur",
            self.cycles.len(),
            value_created,
            to_founder
        );

        self.alchemy
            .mint_token("Auroraium", TokenKind::Fungible, value_created as u64, self.founder_percentage)
            .await;
    }

    pub fn simulate_dynamic_rate(&self) -> f64 {
        let mut rng = rand::thread_rng();
        let rate = 0.01 + (rng.gen::<f64>() * 0.03);
        rate
    }

    pub fn summarize(&self) {
        for (i, c) in self.cycles.iter().enumerate() {
            println!(
                "Cycle {} • {} générés • {:.2} fondateur",
                i + 1,
                c.generated_tokens,
                c.sent_to_founder
            );
        }
    }

    pub fn adjust_founder_share(&mut self, pct: f64) {
        self.founder_percentage = pct.clamp(0.0, 1.0);
    }

    // ✅ Méthodes manquantes
    pub fn initialize(&mut self) {
        println!("[AURORAE++] 🔧 Initialisation économique");
    }

    pub fn innovate(&mut self) {
        println!("[AURORAE++] 🚀 Innovation économique");
    }

    pub fn financial_report(&self) {
        println!(
            "[AURORAE++] 📊 Rapport financier total : {:.2} tokens générés",
            self.total_generated
        );
    }

    pub fn get_total_value(&self) -> f64 {
        self.total_generated
    }

    pub fn add_funds(&mut self, amount: f64) {
        self.total_generated += amount;
    }
}
