//! AURORAE++ - knowledge.rs
//!
//! Base de savoir vivante. Stocke les patterns et insights extraits par le module `learning`
//! pour les rendre accessibles au `generator` et autres composants évolutifs.

use std::collections::{HashMap, HashSet};
use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const DB_PATH: &str = "C:\\Users\\admin\\.github_feed\\aurorae_knowledge.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pattern {
    pub module_name: String,
    pub functions: usize,
    pub structs: usize,
    pub traits: usize,
    pub enums: usize,
}

#[derive(Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub records: Vec<Pattern>,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Memory {
    pub patterns: HashMap<String, Pattern>, // Un dictionnaire pour stocker les patterns par module
    pub insights: HashMap<String, PatternInsight>, // Un dictionnaire pour stocker les insights par module
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternInsight {
    pub module_name: String,
    pub functions: usize,
    pub structs: usize,
    pub traits: usize,
    pub enums: usize,
}

impl KnowledgeBase {
    // Charge la base de données à partir du fichier JSON
    pub fn load() -> Self {
        let path = PathBuf::from(DB_PATH);
        if path.exists() {
            let mut file = File::open(&path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            KnowledgeBase::default()
        }
    }

    // Sauvegarde la base de données dans le fichier JSON
    pub fn save(&self) {
        let path = PathBuf::from(DB_PATH);
        if let Some(parent) = path.parent() {
            let _ = create_dir_all(parent);
        }
        let mut file = File::create(&path).unwrap();
        let json = serde_json::to_string_pretty(self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    // Insère un nouveau pattern dans la base de données
    pub fn insert_pattern(&mut self, pattern: Pattern) {
        self.records.push(pattern);
        self.save();
    }

    // Récupère tous les patterns stockés
    pub fn get_patterns(&self) -> &Vec<Pattern> {
        &self.records
    }

    // Affiche un résumé des patterns stockés dans la base de données
    pub fn summarize(&self) {
        println!("[AURORAE++] Base de savoir : {} projets analysés.", self.records.len());
        for r in &self.records {
            println!(
                "→ {}: {} fn / {} struct / {} trait / {} enum",
                r.module_name, r.functions, r.structs, r.traits, r.enums
            );
        }
    }
}

// Méthode d'intégration avec `learning.rs` pour ajouter des patterns à la base de savoir
impl KnowledgeBase {
    pub fn add_pattern_from_learning(&mut self, pattern: Pattern) {
        self.insert_pattern(pattern);
    }
}

// Méthodes d'intégration avec Memory
impl Memory {
    // Ajouter un pattern à la mémoire
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.insert(pattern.module_name.clone(), pattern);
    }

    // Ajouter un insight à la mémoire
    pub fn add_insight(&mut self, insight: PatternInsight) {
        self.insights.insert(insight.module_name.clone(), insight);
    }

    // Récupérer un pattern par son nom de module
    pub fn get_pattern(&self, module_name: &str) -> Option<&Pattern> {
        self.patterns.get(module_name)
    }

    // Récupérer un insight par son nom de module
    pub fn get_insight(&self, module_name: &str) -> Option<&PatternInsight> {
        self.insights.get(module_name)
    }
}
