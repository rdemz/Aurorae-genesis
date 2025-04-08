// rust_analyzer.rs
//! Module d'analyse de code utilisant rust-analyzer ou une API pour vérifier la qualité du code généré.

use std::process::{Command, Output};

/// Fonction pour analyser le code avec `rust-analyzer` ou un autre analyseur de code.
pub fn analyze(code: &str) -> AnalysisResult {
    // Appel à rust-analyzer via commande (en supposant que rust-analyzer soit installé localement)
    let output: Output = Command::new("rust-analyzer")
        .arg("check") // On utilise la commande `check` pour analyser le code
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("Échec de l'exécution de rust-analyzer");

    // Retourner un résultat d'analyse
    let is_valid = output.status.success();
    let warnings = if !is_valid {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::new()
    };

    AnalysisResult {
        is_valid,
        warnings,
    }
}

/// Structure pour représenter le résultat de l'analyse de code.
pub struct AnalysisResult {
    pub is_valid: bool,
    pub warnings: String,
}
