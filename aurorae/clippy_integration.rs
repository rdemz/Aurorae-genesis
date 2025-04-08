// clippy_integration.rs
//! Intégration de Clippy pour l'analyse du code Rust généré.

use std::process::{Command, Output};

/// Analyse le code avec `clippy` et récupère les avertissements et suggestions.
pub fn run_clippy(code: &str) -> ClippyResult {
    let output: Output = Command::new("cargo")
        .arg("clippy")
        .arg("--")
        .arg("--fix") // Utilise le flag --fix pour appliquer automatiquement les corrections
        .arg("--allow")
        .arg("warnings")
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("Échec de l'exécution de clippy");

    let is_valid = output.status.success();
    let warnings = if !is_valid {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::new()
    };

    ClippyResult {
        is_valid,
        warnings,
    }
}

/// Structure pour représenter le résultat de l'analyse de Clippy.
pub struct ClippyResult {
    pub is_valid: bool,
    pub warnings: String,
}
