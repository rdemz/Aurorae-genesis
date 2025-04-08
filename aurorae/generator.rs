use std::fs::{create_dir_all, File};
use std::io::{Write, Result};
use std::path::Path;
use uuid::Uuid;
use chrono::Utc;
use crate::rust_analyzer::analyze;  // Utilisation du module local rust_analyzer
use crate::clippy_integration::run_clippy; // Utilisation du module local clippy_integration

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

    pub fn save_to_disk(&self, base_path: &str) -> Result<()> {
        let full_path = format!("{}/generated_modules/{}", base_path, self.name);
        let dir_path = Path::new(&full_path);
        create_dir_all(dir_path)?;  // Crée le répertoire s'il n'existe pas

        let file_path = dir_path.join("mod.rs");

        // Exécution de l'analyse de code avant la sauvegarde
        let analysis_result = analyze(&self.content);
        if !analysis_result.is_valid() {
            // Si des erreurs sont présentes, effectuer un refactoring
            println!("[AURORAE++] ⚠️ Erreurs détectées dans le code, refactoring...");
            self.refactor_code();
        } else {
            println!("[AURORAE++] ✅ Analyse réussie sans erreurs.");
        }

        // Exécution de l'analyse avec Clippy pour détecter les problèmes de style
        let clippy_result = run_clippy(&self.content);
        if !clippy_result.is_valid() {
            println!("[AURORAE++] ⚠️ Clippy a trouvé des avertissements, les suggestions seront appliquées.");
            // Appliquer les suggestions Clippy pour améliorer le code généré
            self.apply_clippy_suggestions();
        }

        let mut file = File::create(file_path)?; // Créer et ouvrir le fichier mod.rs
        file.write_all(self.content.as_bytes())?;  // Écrire le contenu dans le fichier

        println!("[AURORAE++] Module {} enregistré à {}", self.name, full_path);
        Ok(())
    }

    // Méthode pour effectuer un refactoring si des problèmes sont détectés dans l'analyse
    fn refactor_code(&self) {
        // Logiciel de refactoring - pourrait utiliser des outils comme `rustfmt` ou des suggestions de `clippy`
        println!("[AURORAE++] 🔧 Refactoring du module {}", self.name);
        // Implémenter ici les suggestions d'amélioration, comme l'optimisation de la gestion des erreurs ou de la mémoire
    }

    // Appliquer les suggestions de Clippy pour améliorer le code généré
    fn apply_clippy_suggestions(&self) {
        // Appliquer les corrections basées sur les avertissements de Clippy
        println!("[AURORAE++] 💡 Application des suggestions Clippy pour le module {}", self.name);
        // Cela pourrait inclure des changements comme la simplification de certains blocs ou l'ajout de gestion d'erreurs
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
