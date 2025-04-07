//! AURORAE++ - mutation.rs
//!
//! Ce module permet à l'IA de modifier ses propres modules générés.
//! Il applique des mutations conscientes sur le code source pour introduire de la variation, de l'amélioration ou des corrections.

use std::fs::{read_to_string, write};
use std::path::Path;
use regex::Regex;
use uuid::Uuid;

#[derive(Debug)]
pub enum MutationResult {
    Success(String),
    NoChanges,
    Error(String),
}

/// Mutations basées sur des patterns simples (exemple : renommer la fonction `hello`)
pub fn mutate_module_code(path: &str) -> MutationResult {
    let mod_path = format!("{}/mod.rs", path);
    let code_path = Path::new(&mod_path);

    if !code_path.exists() {
        return MutationResult::Error("mod.rs non trouvé".into());
    }

    match read_to_string(code_path) {
        Ok(content) => {
            let mut mutated = content.clone();

            // Exemple : remplacer fn hello() par fn evolved_hello()
            let re = Regex::new(r"fn\s+hello\s*\(").unwrap();
            mutated = re.replace_all(&mutated, "fn evolved_hello(").to_string();

            if mutated != content {
                if let Err(e) = write(code_path, &mutated) {
                    return MutationResult::Error(format!("Erreur d'écriture: {}", e));
                }
                MutationResult::Success(Uuid::new_v4().to_string())
            } else {
                MutationResult::NoChanges
            }
        }
        Err(e) => MutationResult::Error(format!("Erreur lecture: {}", e)),
    }
}
