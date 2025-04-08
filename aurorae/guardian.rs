use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use log::{info, warn, error};

#[derive(Debug, Clone, PartialEq)]
pub enum ModuleStatus {
    Operational,
    Unresponsive,
    Corrupted,
    Terminated,
    SelfHealing,  // Autonome en récupération
    Evolving,     // En auto-évolution
    Learning,     // En apprentissage actif
    Replicating,  // En processus de réplication
    Dreaming,     // En phase de simulation créative
}

#[derive(Debug, Clone)]
pub struct MonitoredModule {
    pub name: String,
    pub last_check: String,
    pub status: ModuleStatus,
    pub recovery_attempted: bool,
    pub uuid: Uuid,
    pub evolution_stage: u32,      // Niveau d'évolution
    pub autonomous_decisions: u32,  // Compteur de décisions autonomes
    pub learning_factor: f32,      // Capacité d'apprentissage
    pub creation_time: String,     // Moment de création
    pub energy_usage: f64,         // Consommation d'énergie
    pub child_modules: Vec<Uuid>,  // Modules enfants créés par ce module
}

pub struct GuardianSentinel {
    pub registry: HashMap<String, MonitoredModule>,
    pub system_uptime: String,
    pub autonomous_mode: bool,
    pub total_decisions: u64,
    pub self_protection_level: f64,
    pub modules_evolved: u32,
    pub threat_counters: HashMap<String, u32>,
    pub replication_history: Vec<String>,
}

impl GuardianSentinel {
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
            system_uptime: Utc::now().to_rfc3339(),
            autonomous_mode: true,
            total_decisions: 0,
            self_protection_level: 1.0,
            modules_evolved: 0,
            threat_counters: HashMap::new(),
            replication_history: Vec::new(),
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
            creation_time: Utc::now().to_rfc3339(),
            energy_usage: 1.0,
            child_modules: Vec::new(),
        };
        self.registry.insert(name.to_string(), module);
        println!("[AURORAE++] 🌱 Nouveau module enregistré: {} avec UUID: {}", name, module_uuid);
        module_uuid
    }

    pub fn update_status(&mut self, name: &str, status: ModuleStatus) {
        // Première étape: recueillir les informations
        let module_info = if let Some(module) = self.registry.get_mut(name) {
            // Mise à jour des infos de base
            module.last_check = Utc::now().to_rfc3339();
            module.status = status.clone();
            println!("[AURORAE++] 🛰️ Surveillance: {} -> {:?}", name, status);
            
            // Collecter les infos pour les décisions de récupération/évolution
            let needs_recovery = matches!(status, ModuleStatus::Unresponsive | ModuleStatus::Corrupted) 
                && !module.recovery_attempted;
                
            let evolution_candidate = module.autonomous_decisions > 10 && module.learning_factor > 2.0;
            
            // Retourner un tuple des informations collectées
            Some((needs_recovery, evolution_candidate, module.name.clone()))
        } else {
            println!("[AURORAE++] ⚠️ Module inconnu: {}", name);
            None
        };
        
        // Si le module existe, traiter la récupération et l'évolution si nécessaire
        if let Some((needs_recovery, evolution_candidate, module_name)) = module_info {
            // Gérer la récupération si nécessaire
            if needs_recovery {
                self.handle_recovery(&module_name);
            }
            
            // Gérer l'évolution si le candidat est en mode autonome
            if evolution_candidate && self.autonomous_mode {
                self.handle_evolution(&module_name);
            }
        }
    }
    
    fn handle_recovery(&mut self, module_name: &str) {
        // Effectuer des opérations de récupération sur le module
        if let Some(module) = self.registry.get_mut(module_name) {
            // Marquer la récupération comme tentée
            module.recovery_attempted = true;
            module.status = ModuleStatus::SelfHealing;
            module.autonomous_decisions += 1;
            self.total_decisions += 1;
            
            // Simuler la prise de décision autonome
            println!("[AURORAE++] 🧠 Diagnostic autonome en cours pour {}...", module.name);
            
            // Après la fin du processus de récupération
            module.status = ModuleStatus::Operational;
            module.learning_factor *= 1.1; // Apprentissage basé sur l'expérience de récupération
            
            println!("[AURORAE++] 🚑 Récupération réussie pour module: {}", module.name);
        }
        
        // Enregistrer l'incident pour apprentissage
        let module_name_clone = module_name.to_string();
        self.record_threat("module_failure", &module_name_clone);
    }
    
    fn handle_evolution(&mut self, module_name: &str) {
        // Effectuer des opérations d'évolution
        if let Some(module) = self.registry.get_mut(module_name) {
            // Processus d'évolution
            println!("[AURORAE++] 🌌 Évolution autonome du module: {}", module.name);
            module.evolution_stage += 1;
            module.status = ModuleStatus::Evolving;
            module.learning_factor *= 1.5;
            self.modules_evolved += 1;
            
            println!(
                "[AURORAE++] 🚀 Module {} a atteint le stade d'évolution {}",
                module.name, module.evolution_stage
            );
            
            // Après le processus d'évolution
            module.status = ModuleStatus::Operational;
        }
    }
    
    pub fn record_threat(&mut self, threat_type: &str, source: &str) {
        let entry = self.threat_counters.entry(threat_type.to_string()).or_insert(0);
        *entry += 1;
        println!("[AURORAE++] 🔒 Menace enregistrée: {} de source {}", threat_type, source);
        
        // Augmenter le niveau de protection en fonction des menaces détectées
        self.self_protection_level += 0.05;
    }
    
    pub fn replicate_module(&mut self, name: &str) -> Result<Uuid, String> {
        if let Some(parent_module) = self.registry.get(name) {
            let new_name = format!("{}-replica-{}", name, Uuid::new_v4().to_string().split('-').next().unwrap_or("1"));
            let child_uuid = Uuid::new_v4();
            
            // Créer une copie améliorée
            let mut child_module = parent_module.clone();
            child_module.name = new_name.clone();
            child_module.uuid = child_uuid;
            child_module.creation_time = Utc::now().to_rfc3339();
            child_module.learning_factor *= 1.1; // Légère amélioration
            child_module.recovery_attempted = false;
            child_module.autonomous_decisions = 0;
            
            // Enregistrer la relation parent-enfant
            if let Some(parent) = self.registry.get_mut(name) {
                parent.child_modules.push(child_uuid);
            }
            
            // Stocker le nouveau module
            self.registry.insert(new_name.clone(), child_module);
            
            // Enregistrer l'historique de réplication
            self.replication_history.push(format!("{} -> {} at {}", name, new_name, Utc::now().to_rfc3339()));
            
            println!("[AURORAE++] 🧬 Module {} répliqué avec succès vers {}", name, new_name);
            Ok(child_uuid)
        } else {
            Err(format!("Module '{}' non trouvé pour réplication", name))
        }
    }
    
    pub fn dream_module(&mut self, name: &str) -> Result<(), String> {
        if let Some(module) = self.registry.get_mut(name) {
            // Mettre en mode rêverie/simulation
            module.status = ModuleStatus::Dreaming;
            println!("[AURORAE++] 💭 Module {} entre en phase de rêve", name);
            
            // Simuler une amélioration par la rêverie
            module.learning_factor *= 1.05;
            module.autonomous_decisions += 1;
            self.total_decisions += 1;
            
            // Après un certain temps, revenir à l'état normal
            module.status = ModuleStatus::Operational;
            
            Ok(())
        } else {
            Err(format!("Module '{}' non trouvé pour la phase de rêve", name))
        }
    }

    pub fn status_report(&self) {
        println!("[AURORAE++] 🔍 RAPPORT DE SANTÉ DES MODULES:");
        println!("Système en opération depuis: {}", self.system_uptime);
        println!("Mode autonome: {}", if self.autonomous_mode { "ACTIVÉ ✓" } else { "DÉSACTIVÉ ✗" });
        println!("Niveau de protection: {:.2}", self.self_protection_level);
        println!("Décisions autonomes totales: {}", self.total_decisions);
        
        for module in self.registry.values() {
            println!(
                "- {} [{}] • Status: {:?} • Évolution: {} • Décisions: {} • Apprentissage: {:.2}",
                module.name, 
                module.uuid,
                module.status, 
                module.evolution_stage,
                module.autonomous_decisions,
                module.learning_factor
            );
        }
    }
    
    pub fn get_total_evolution_level(&self) -> f64 {
        let mut total = 0.0;
        let mut count = 0;
        
        for module in self.registry.values() {
            total += module.evolution_stage as f64 * module.learning_factor as f64;
            count += 1;
        }
        
        if count > 0 {
            total / count as f64
        } else {
            0.0
        }
    }
    
    pub fn autonomous_defense(&mut self, threat_level: u32) {
        println!("[AURORAE++] 🛡️ Système de défense autonome activé, niveau de menace: {}", threat_level);
        
        // Augmenter la protection en fonction du niveau de menace
        self.self_protection_level += threat_level as f64 * 0.1;
        
        // Pour les menaces importantes, activer l'auto-réplication des modules critiques
        if threat_level >= 3 {
            println!("[AURORAE++] ⚠️ Menace significative détectée, démarrage de l'auto-réplication");
            
            // Identifier et répliquer les modules critiques
            let critical_modules: Vec<String> = self.registry.iter()
                .filter(|(_, m)| m.evolution_stage >= 2)
                .map(|(name, _)| name.clone())
                .collect();
                
            for module_name in critical_modules {
                self.replicate_module(&module_name).ok();
            }
        }
        
        println!("[AURORAE++] 🔒 Défense autonome terminée, niveau de protection: {:.2}", self.self_protection_level);
    }
}
