// refactor.rs
//! Module de refactoring automatique pour optimiser le code généré et appliquer des règles de formatage.

use std::process::Command;

/// Effectue un refactoring du code en utilisant `rustfmt` pour appliquer des règles de formatage.
pub fn refactor_code(code: &str) -> String {
    // Appel à rustfmt pour formater le code
    let output = Command::new("rustfmt")
        .arg("--emit=stdout")
        .stdin(std::process::Stdio::piped())
        .output()
        .expect("Échec de l'exécution de rustfmt");

    // Retourner le code formaté
    String::from_utf8_lossy(&output.stdout).to_string()
}
