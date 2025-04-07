//! AURORAE++ - validator.rs
//!
//! Ce module compile et teste dynamiquement les fichiers générés par `generator.rs`
//! pour s'assurer qu'ils sont valides, sécurisés et exécutables.

use std::process::Command;
use std::path::Path;
use std::fs;

/// Résultat d'une validation de module
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Success,
    CompileError(String),
    MissingCargo,
    InternalError(String),
}

/// Valide un module généré en lançant un `cargo check`
pub fn validate_generated_module(path: &str) -> ValidationResult {
    let module_path = Path::new(path);
    if !module_path.exists() {
        return ValidationResult::InternalError("Chemin invalide".to_string());
    }

    // Crée un Cargo.toml minimal si nécessaire
    let cargo_path = module_path.join("Cargo.toml");
    if !cargo_path.exists() {
        if let Err(_e) = fs::write(&cargo_path, minimal_cargo_toml()) {
            return ValidationResult::InternalError(format!("Erreur écriture Cargo.toml: {}", e));
        }
    }

    // Lance la commande cargo check
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(module_path)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                ValidationResult::Success
            } else {
                let err = String::from_utf8_lossy(&result.stderr).to_string();
                ValidationResult::CompileError(err)
            }
        }
        Err(e) => ValidationResult::MissingCargo,
    }
}

/// Génére un `Cargo.toml` minimal pour compiler un module isolé
fn minimal_cargo_toml() -> &'static str {
    "[package]\nname = \"generated_module\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n"
}
