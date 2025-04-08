// pattern_extractor.rs
//! Module pour extraire des patterns de code à partir de dépôts GitHub et les enregistrer dans `knowledge.rs`.

use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::Read;
use crate::knowledge::{Memory, Pattern};
use regex::Regex;

/// Extrait des patterns de code à partir d'un dossier de fichiers Rust.
pub fn extract_patterns_from_directory(dir: &Path) -> Vec<Pattern> {
    let mut patterns = Vec::new();
    let files = find_rust_files(dir);

    let pattern_regex = Regex::new(r"(?m)^\s*pub\s*(fn|struct|trait|enum)\s+").unwrap();

    for file in files {
        let content = read_file_content(&file);
        let matches = pattern_regex.find_iter(&content);

        for _ in matches {
            patterns.push(Pattern {
                module_name: file.to_string_lossy().to_string(),
                functions: 0, // Vous pouvez affiner ces valeurs plus tard
                structs: 0,
                traits: 0,
                enums: 0,
            });
        }
    }

    patterns
}

/// Récupère tous les fichiers `.rs` dans un répertoire
fn find_rust_files(base: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    if let Ok(entries) = read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false) {
                results.push(path);
            }
        }
    }
    results
}

/// Lit le contenu d'un fichier
fn read_file_content(file: &PathBuf) -> String {
    let mut file_content = String::new();
    let mut file = File::open(file).unwrap();
    file.read_to_string(&mut file_content).unwrap();
    file_content
}
