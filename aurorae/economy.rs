use chrono::Utc;
use crate::founder_income::reward_founder;

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
    pub founder_percentage: f64, // ex: 0.15 for 15%
}

impl EconomyEngine {
    pub fn new() -> Self {
        Self {
            cycles: vec![],
            total_generated: 0.0,
            founder_percentage: 0.15,
        }
    }

    pub fn simulate_cycle(&mut self, value_created: f64) {
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
    }

    pub fn summarize(&self) {
        println!("[AURORAE++] 🌐 TOTAL GÉNÉRÉ : {:.4} tokens", self.total_generated);
        for (i, c) in self.cycles.iter().enumerate() {
            println!("→ Cycle {} • {} tokens créés • {:.2} au fondateur", i + 1, c.generated_tokens, c.sent_to_founder);
        }
    }

    pub fn adjust_founder_share(&mut self, pct: f64) {
        self.founder_percentage = pct.clamp(0.0, 1.0);
    }
}
