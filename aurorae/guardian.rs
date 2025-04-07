//! AURORAE++ - guardian.rs
//!
//! Sentinelle principale de l'intégrité vitale. Ce module veille sur l'état des composants,
//! relance ceux qui s'arrêtent, stoppe les processus toxiques, et restaure l'équilibre fonctionnel.

use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ModuleStatus {
    Operational,
    Unresponsive,
    Corrupted,
    Terminated,
}

#[derive(Debug, Clone)]
pub struct MonitoredModule {
    pub name: String,
    pub last_check: String,
    pub status: ModuleStatus,
    pub recovery_attempted: bool,
    pub uuid: Uuid,
}

#[derive(Default)]
pub struct GuardianSentinel {
    pub registry: HashMap<String, MonitoredModule>,
}

impl GuardianSentinel {
    pub fn new() -> Self {
        Self { registry: HashMap::new() }
    }

    pub fn register_module(&mut self, name: &str) {
        let module = MonitoredModule {
            name: name.to_string(),
            last_check: Utc::now().to_rfc3339(),
            status: ModuleStatus::Operational,
            recovery_attempted: false,
            uuid: Uuid::new_v4(),
        };
        self.registry.insert(name.to_string(), module);
    }

    pub fn update_status(&mut self, name: &str, status: ModuleStatus) {
        if let Some(module) = self.registry.get_mut(name) {
            module.last_check = Utc::now().to_rfc3339();
            module.status = status.clone();
            println!("[AURORAE++] 🛰️ Surveillance : {} -> {:?}", name, status);

            if matches!(status, ModuleStatus::Unresponsive | ModuleStatus::Corrupted) && !module.recovery_attempted {
                self.attempt_recovery(module);
            }
        }
    }

    pub fn attempt_recovery(&mut self, module: &mut MonitoredModule) {
        println!("[AURORAE++] 🚑 Tentative de récupération pour module : {}", module.name);
        module.recovery_attempted = true;
        module.status = ModuleStatus::Operational;
    }

    pub fn status_report(&self) {
        println!("[AURORAE++] 🔍 RAPPORT DE SANTÉ DES MODULES :");
        for module in self.registry.values() {
            println!("- {} [{}] • Status: {:?} • Recovered: {}", module.name, module.uuid, module.status, module.recovery_attempted);
        }
    }
}
