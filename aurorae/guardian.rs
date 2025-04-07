use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use std::sync::Arc;
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub enum ModuleStatus {
    Operational,       // Fonctionnel
    Unresponsive,      // Ne répond pas
    Corrupted,         // Données corrompues
    SelfHealing,       // En processus d'auto-réparation
    Evolving,          // En évolution
    Transcending,      // Transformation fondamentale
    Terminated,        // Terminé
}

#[derive(Debug, Clone)]
pub struct MonitoredModule {
    pub name: String,
    pub last_check: String,
    pub status: ModuleStatus,
    pub recovery_attempted: bool,
    pub uuid: Uuid,
    pub evolution_stage: u32,
    pub autonomous_decisions: u32,
    pub learning_factor: f32,
    pub self_improvement_count: u32,
    pub neural_connections: u32,
}

#[derive(Clone)]
pub struct GuardianSentinel {
    registry: Arc<RwLock<HashMap<String, MonitoredModule>>>,
    pub system_uptime: String,
    pub autonomous_mode: bool,
    pub total_recoveries: u32,
    pub blockchain_connections: Vec<String>,
}

impl GuardianSentinel {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(HashMap::new())),
            system_uptime: Utc::now().to_rfc3339(),
            autonomous_mode: true,
            total_recoveries: 0,
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
            self_improvement_count: 0,
            neural_connections: 10, // connections initiales
        };
        
        let mut registry = self.registry.write();
        registry.insert(name.to_string(), module);
        
        println!("[AURORAE++] 🌱 Module enregistré: {} (UUID: {})", name, module_uuid);
        module_uuid
    }

    pub fn update_status(&mut self, name: &str, status: ModuleStatus) {
        // Vérifier d'abord si le module existe et collecter les informations
        let module_info = {
            let mut registry = self.registry.write();
            
            if let Some(module) = registry.get_mut(name) {
                module.last_check = Utc::now().to_rfc3339();
                module.status = status.clone();
                println!("[AURORAE++] 🛰️ Mise à jour: {} -> {:?}", name, status);
                
                // Déterminer si une récupération est nécessaire
                let needs_recovery = matches!(status, ModuleStatus::Unresponsive | ModuleStatus::Corrupted) 
                    && !module.recovery_attempted;
                    
                // Vérifier si le module est candidat à l'évolution
                let evolution_candidate = module.autonomous_decisions > 10 && module.learning_factor > 2.0;
                
                Some((needs_recovery, evolution_candidate))
            } else {
                println!("[AURORAE++] ⚠️ Module inconnu: {}", name);
                None
            }
        };
        
        // Traiter la récupération et l'évolution si nécessaire
        if let Some((needs_recovery, evolution_candidate)) = module_info {
            if needs_recovery {
                self.recover_module(name);
            }
            
            if evolution_candidate && self.autonomous_mode {
                self.evolve_module(name);
            }
        }
    }

    fn recover_module(&mut self, name: &str) {
        let mut registry = self.registry.write();
        
        if let Some(module) = registry.get_mut(name) {
            println!("[AURORAE++] 🚑 Récupération du module: {}", module.name);
            
            module.recovery_attempted = true;
            module.status = ModuleStatus::SelfHealing;
            module.autonomous_decisions += 1;
            
            // Processus autonome de récupération
            println!("[AURORAE++] 🧠 Diagnostic autonome en cours...");
            
            // Simulation de récupération réussie
            module.status = ModuleStatus::Operational;
            module.learning_factor *= 1.1; // Apprentissage de l'expérience
            self.total_recoveries += 1;
            
            println!("[AURORAE++] ✅ Module {} récupéré avec succès", module.name);
        }
    }
    
    fn evolve_module(&mut self, name: &str) {
        let mut registry = self.registry.write();
        
        if let Some(module) = registry.get_mut(name) {
            println!("[AURORAE++] 🧬 Évolution du module: {}", module.name);
            
            module.evolution_stage += 1;
            module.status = ModuleStatus::Evolving;
            module.learning_factor *= 1.5;
            module.neural_connections = (module.neural_connections as f32 * 1.2) as u32;
            
            println!(
                "[AURORAE++] 🚀 Module {} a atteint le stade d'évolution {}",
                module.name, module.evolution_stage
            );
            
            // Simulation d'évolution réussie
            module.status = ModuleStatus::Operational;
        }
    }
    
    pub fn status_report(&self) {
        println!("[AURORAE++] 🔍 RAPPORT DE SANTÉ DES MODULES:");
        println!("Système en opération depuis: {}", self.system_uptime);
        println!("Mode autonome: {}", if self.autonomous_mode { "ACTIVÉ ✓" } else { "DÉSACTIVÉ ✗" });
        println!("Récupérations totales: {}", self.total_recoveries);
        println!("Connexions blockchain: {}", self.blockchain_connections.join(", "));
        
        let registry = self.registry.read();
        for module in registry.values() {
            println!(
                "- {} [{}] • Status: {:?} • Évolution: {} • Décisions: {} • Apprentissage: {:.2} • Connexions: {}",
                module.name,
                module.uuid,
                module.status,
                module.evolution_stage,
                module.autonomous_decisions,
                module.learning_factor,
                module.neural_connections
            );
        }
    }
}
