use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ModuleStatus {
    Operational,
    Unresponsive,
    Corrupted,
    Terminated,
    SelfHealing,  // Added for autonomous recovery
    Evolving,     // Added for self-improvement capability
}

#[derive(Debug, Clone)]
pub struct MonitoredModule {
    pub name: String,
    pub last_check: String,
    pub status: ModuleStatus,
    pub recovery_attempted: bool,
    pub uuid: Uuid,
    pub evolution_stage: u32,      // Track evolution progress
    pub autonomous_decisions: u32,  // Count decisions made independently
    pub learning_factor: f32,      // Measure of learning capability
}

#[derive(Default)]
pub struct GuardianSentinel {
    pub registry: HashMap<String, MonitoredModule>,
    pub system_uptime: String,
    pub autonomous_mode: bool,
    pub blockchain_connections: Vec<String>,
}

impl GuardianSentinel {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            system_uptime: Utc::now().to_rfc3339(),
            autonomous_mode: true,
            blockchain_connections: Vec::new(),
        }
    }

    pub fn register_module(&mut self, name: &str) -> Uuid {
        let module_uuid = Uuid::new_v4();
        let module = MonitoredModule {
            name: name.to_string(),
            last_check: Utc::now().to_rfc3339(),
            status: ModuleStatus::Operational,
            recovery_attempted: false,
            uuid: module_uuid,
            evolution_stage: 1,
            autonomous_decisions: 0,
            learning_factor: 1.0,
        };
        self.registry.insert(name.to_string(), module);
        println!("[AURORAE++] 🌱 Nouveau module enregistré: {} avec UUID: {}", name, module_uuid);
        module_uuid
    }

    pub fn update_status(&mut self, name: &str, status: ModuleStatus) {
        // First determine if recovery is needed, within a limited scope
        let recovery_needed = {
            if let Some(module) = self.registry.get_mut(name) {
                module.last_check = Utc::now().to_rfc3339();
                module.status = status.clone();
                println!("[AURORAE++] 🛰️ Surveillance: {} -> {:?}", name, status);

                // Return whether recovery is needed
                matches!(status, ModuleStatus::Unresponsive | ModuleStatus::Corrupted)
                    && !module.recovery_attempted
            } else {
                println!("[AURORAE++] ⚠️ Module inconnu: {}", name);
                return;
            }
        }; // First mutable borrow ends here

        // Now we can safely borrow self mutably again if recovery is needed
        if recovery_needed {
            if let Some(module) = self.registry.get_mut(name) {
                self.attempt_recovery(module);
            }
        }
        
        // Autonomous evolution check (in a separate scope)
        let evolution_candidate = {
            if let Some(module) = self.registry.get(name) {
                module.autonomous_decisions > 10 && module.learning_factor > 2.0
            } else {
                false
            }
        };
        
        if evolution_candidate && self.autonomous_mode {
            if let Some(module) = self.registry.get_mut(name) {
                self.evolve_module(module);
            }
        }
    }

    pub fn attempt_recovery(&mut self, module: &mut MonitoredModule) {
        println!(
            "[AURORAE++] 🚑 Tentative de récupération pour module: {}",
            module.name
        );
        module.recovery_attempted = true;
        module.status = ModuleStatus::SelfHealing;
        module.autonomous_decisions += 1;
        
        // Simulate autonomous decision making
        println!("[AURORAE++] 🧠 Diagnostic autonome en cours...");
        
        // After recovery process completes
        module.status = ModuleStatus::Operational;
        module.learning_factor *= 1.1; // Learning from recovery experience
    }
    
    pub fn evolve_module(&mut self, module: &mut MonitoredModule) {
        println!(
            "[AURORAE++] 🌌 Évolution autonome du module: {}",
            module.name
        );
        module.evolution_stage += 1;
        module.status = ModuleStatus::Evolving;
        module.learning_factor *= 1.5;
        
        println!(
            "[AURORAE++] 🚀 Module {} a atteint le stade d'évolution {}",
            module.name, module.evolution_stage
        );
        
        // After evolution process
        module.status = ModuleStatus::Operational;
    }
    
    pub fn connect_blockchain(&mut self, chain_id: &str) {
        println!("[AURORAE++] 🔗 Connexion à la blockchain: {}", chain_id);
        self.blockchain_connections.push(chain_id.to_string());
    }
    
    pub fn autonomous_decision(&mut self, module_name: &str, decision_type: &str) {
        if let Some(module) = self.registry.get_mut(module_name) {
            module.autonomous_decisions += 1;
            println!(
                "[AURORAE++] 🤖 Décision autonome #{} pour {}: {}",
                module.autonomous_decisions, module.name, decision_type
            );
        }
    }

    pub fn status_report(&self) {
        println!("[AURORAE++] 🔍 RAPPORT DE SANTÉ DES MODULES:");
        println!("Système en opération depuis: {}", self.system_uptime);
        println!("Mode autonome: {}", if self.autonomous_mode { "ACTIVÉ ✓" } else { "DÉSACTIVÉ ✗" });
        println!("Connexions blockchain: {}", self.blockchain_connections.join(", "));
        
        for module in self.registry.values() {
            println!(
                "- {} [{}] • Status: {:?} • Récupération: {} • Évolution: {} • Décisions: {} • Apprentissage: {:.2}",
                module.name, 
                module.uuid,
                module.status, 
                module.recovery_attempted,
                module.evolution_stage,
                module.autonomous_decisions,
                module.learning_factor
            );
        }
    }
    
    pub fn self_improve(&mut self) {
        println!("[AURORAE++] 🧬 Système en auto-amélioration");
        // Implementation would involve code that modifies its own behavior
        // This is conceptual as actual self-modifying code would require
        // more complex mechanisms
