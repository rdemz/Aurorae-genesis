use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct AuroraInstance {
    pub id: Uuid,
    pub created_at: String,
    pub purpose: String,
    pub inherited_modules: Vec<String>,
    pub is_active: bool,
}

#[derive(Default)]
pub struct ReproductionEngine {
    pub children: Vec<AuroraInstance>,
}

impl ReproductionEngine {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn spawn_instance(&mut self, purpose: &str, modules: Vec<&str>) -> AuroraInstance {
        let instance = AuroraInstance {
            id: Uuid::new_v4(),
            created_at: Utc::now().to_rfc3339(),
            purpose: purpose.to_string(),
            inherited_modules: modules.iter().map(|s| s.to_string()).collect(),
            is_active: true,
        };

        println!("[AURORAE++] 🧫 Nouvelle instance créée : {} • But: {}", instance.id, purpose);
        self.children.push(instance.clone());
        instance
    }

    pub fn list_instances(&self) {
        println!("[AURORAE++] 🌱 ENFANTS ACTIFS : {}", self.children.len());
        for i in &self.children {
            println!("- {} • Purpose: {} • Modules: {:?}", i.id, i.purpose, i.inherited_modules);
        }
    }
}
