//! AURORAE++ - learning.rs
//!
//! Ce module lit les dépôts clonés localement, analyse les fichiers `.rs`, et extrait des patterns de code
//! pour enrichir la mémoire vivante (knowledge.rs) et guider le générateur.

use std::fs::{read_to_string, read_dir};
use std::path::{Path, PathBuf};
use regex::Regex;
use std::collections::HashMap;
use crate::knowledge::{KnowledgeBase, Pattern}; // Importer KnowledgeBase et Pattern du module knowledge

const FEED_PATH: &str = "C:\\Users\\admin\\.github_feed";

#[derive(Debug, Clone)]
pub struct PatternInsight {
    pub module_name: String,
    pub functions: usize,
    pub structs: usize,
    pub traits: usize,
    pub enums: usize,
}

impl PatternInsight {
    // Convertir les insights en objets Pattern pour les stocker dans la mémoire
    pub fn to_pattern(self) -> Pattern {
        Pattern {
            module_name: self.module_name,
            functions: self.functions,
            structs: self.structs,
            traits: self.traits,
            enums: self.enums,
        }
    }
}

/// Lit tous les projets clonés et analyse leurs fichiers Rust
pub fn scan_feed_and_learn(knowledge_base: &mut KnowledgeBase) {
    if let Ok(entries) = read_dir(FEED_PATH) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let module_name = path.file_name().unwrap().to_string_lossy().to_string();
                let stats = analyze_rust_files(&path);
                let pattern = stats.to_pattern();
                
                // Ajouter les patterns extraits à la mémoire vivante
                knowledge_base.add_pattern(pattern);  // Correction ici pour utiliser `knowledge_base`
            }
        }
    }
}

/// Analyse tous les fichiers .rs dans un répertoire
fn analyze_rust_files(dir: &Path) -> PatternInsight {
    let mut count = HashMap::new();
    count.insert("fn", 0);
    count.insert("struct", 0);
    count.insert("trait", 0);
    count.insert("enum", 0);

    let rs_files = find_rust_files(dir);
    let re_map: HashMap<&str, Regex> = [
        ("fn", Regex::new(r"(?m)^\s*pub\s*fn\s+").unwrap()),
        ("struct", Regex::new(r"(?m)^\s*pub\s*struct\s+").unwrap()),
        ("trait", Regex::new(r"(?m)^\s*pub\s*trait\s+").unwrap()),
        ("enum", Regex::new(r"(?m)^\s*pub\s*enum\s+").unwrap()),
    ].iter().cloned().collect();

    for file in rs_files {
        if let Ok(content) = read_to_string(&file) {
            for (key, regex) in &re_map {
                let matches = regex.find_iter(&content).count();
                *count.get_mut::<&str>(key).unwrap() += matches;
            }
        }
    }

    PatternInsight {
        module_name: "anonymous".to_string(),
        functions: *count.get("fn").unwrap(),
        structs: *count.get("struct").unwrap(),
        traits: *count.get("trait").unwrap(),
        enums: *count.get("enum").unwrap(),
    }
}

/// Récupère tous les fichiers .rs d’un projet donné
fn find_rust_files(base: &Path) -> Vec<PathBuf> {
    let mut results = vec![];
    if let Ok(entries) = read_dir(base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map(|ext| ext == "rs").unwrap_or(false) {
                results.push(path);
            } else if path.is_dir() {
                results.extend(find_rust_files(&path));
            }
        }
    }
    results
}
