//! AURORAE++ - generator.rs
//!
//! Moteur de génération de code Rust vivant. 
//! Ce module répond aux signaux envoyés par brain.rs pour produire du code réel, structuré, compilable et évolutif.

use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug)]
pub struct GeneratedModule {
    pub id: Uuid,
    pub name: String,
    pub content: String,
    pub timestamp: String,
}

impl GeneratedModule {
    pub fn new(name: &str, content: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            content: content.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn save_to_disk(&self, base_path: &str) -> std::io::Result<()> {
        let full_path = format!("{}/generated_modules/{}", base_path, self.name);
        let dir_path = Path::new(&full_path);
        create_dir_all(dir_path)?;

        let file_path = dir_path.join("mod.rs");
        let mut file = File::create(file_path)?;
        file.write_all(self.content.as_bytes())?;

        println!("[AURORAE++] Module {} enregistré à {}", self.name, full_path);
        Ok(())
    }
}

/// Génère un module Rust de base intelligent
pub fn generate_basic_module(name: &str) -> GeneratedModule {
    let content = format!(
        "// Module généré automatiquement par AURORAE++\n\n\
/// Nom : {}\n/// UID : {}\n\npub fn hello() {{\n    println!(\"[{}] Hello from generated module!\");\n}}",
        name,
        Uuid::new_v4(),
        name.to_uppercase()
    );

    GeneratedModule::new(name, &content)
}

/// Lance une génération complète
pub fn trigger_generation(base_path: &str, name: &str) {
    let module = generate_basic_module(name);
    if let Err(e) = module.save_to_disk(base_path) {
        eprintln!("[AURORAE++] Échec de la sauvegarde du module {}: {}", name, e);
    }
}
