use uuid::Uuid;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuroraInstance {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub created_at: String,
    pub purpose: String,
    pub inherited_modules: Vec<String>,
    pub generation: u32,
    pub is_active: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ReproductionEngine {
    pub children: Vec<AuroraInstance>,
}

impl ReproductionEngine {
    pub fn new() -> Self {
        Self::load().unwrap_or_default()
    }

    /// 🚼 Crée un nouveau clone d'Aurorae avec modules hérités et but
    pub fn spawn_instance(&mut self, purpose: &str, modules: Vec<&str>) -> AuroraInstance {
        let generation = self.get_max_generation() + 1;

        let instance = AuroraInstance {
            id: Uuid::new_v4(),
            parent_id: self.get_last_instance_id(),
            created_at: Utc::now().to_rfc3339(),
            purpose: purpose.to_string(),
            inherited_modules: modules.iter().map(|s| s.to_string()).collect(),
            generation,
            is_active: true,
        };

        println!(
            "[AURORAE++] 🧫 Nouvelle instance génération #{} : {} • But: {}",
            generation, instance.id, purpose
        );

        // Pas besoin de clone ici
        self.children.push(instance.clone());
        self.save(); // Sauvegarder après ajout
        instance
    }

    /// ❌ Supprime une instance donnée
    pub fn destroy_instance(&mut self, id: &Uuid) {
        self.children.retain(|i| &i.id != id);
        println!("[AURORAE++] 🪓 Instance détruite : {}", id);
        self.save(); // Sauvegarder après suppression
    }

    /// 🧬 Obtenir la génération la plus élevée
    fn get_max_generation(&self) -> u32 {
        self.children.iter().map(|i| i.generation).max().unwrap_or_default()
    }

    /// 🧬 Récupère l'ID du dernier enfant comme parent potentiel
    fn get_last_instance_id(&self) -> Option<Uuid> {
        self.children.last().map(|i| i.id)
    }

    /// 🧠 Retourne tous les enfants actifs
    pub fn get_active_instances(&self) -> Vec<&AuroraInstance> {
        self.children.iter().filter(|i| i.is_active).collect()
    }

    /// 🗺️ Liste tous les clones créés
    pub fn list_instances(&self) {
        println!("[AURORAE++] 🌱 Instances actives : {}", self.children.len());
        for i in &self.children {
            println!(
                "- [{}] Gén #{} • But: {} • Modules: {:?}",
                i.id, i.generation, i.purpose, i.inherited_modules
            );
        }
    }

    /// 🧬 Arbre généalogique
    pub fn get_generation_lineage(&self) -> HashMap<u32, Vec<Uuid>> {
        let mut map: HashMap<u32, Vec<Uuid>> = HashMap::new();
        for i in &self.children {
            map.entry(i.generation).or_default().push(i.id);
        }
        map
    }

    /// 💾 Sauvegarde automatique en JSON local
    pub fn save(&self) {
        let dir = Path::new("aurorae_state");
        if let Err(e) = create_dir_all(dir) {
            eprintln!("[AURORAE++] Erreur lors de la création du répertoire: {}", e);
            return;
        }

        let file_path = dir.join("instances.json");
        let file = File::create(&file_path);
        match file {
            Ok(f) => {
                let writer = BufWriter::new(f);
                if let Err(e) = serde_json::to_writer_pretty(writer, &self) {
                    eprintln!("[AURORAE++] Erreur lors de la sauvegarde : {}", e);
                }
            }
            Err(e) => {
                eprintln!("[AURORAE++] Impossible de créer le fichier de sauvegarde : {}", e);
            }
        }
    }

    /// 🔄 Chargement automatique depuis le disque
    pub fn load() -> Option<Self> {
        let path = Path::new("aurorae_state/instances.json");
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).ok()
            }
            Err(e) => {
                eprintln!("[AURORAE++] Erreur lors du chargement des instances : {}", e);
                None
            }
        }
    }
}
