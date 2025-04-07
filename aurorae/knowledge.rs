//! AURORAE++ - knowledge.rs
//!
//! Base de savoir vivante. Stocke les patterns et insights extraits par le module `learning`
//! pour les rendre accessibles au `generator` et autres composants évolutifs.

use std::collections::HashMap;
use std::fs::{File, create_dir_all};
use std::io::{Write, Read};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

const DB_PATH: &str = "C:\\Users\\admin\\.github_feed\\aurorae_knowledge.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatternStat {
    pub project: String,
    pub functions: usize,
    pub structs: usize,
    pub traits: usize,
    pub enums: usize,
}

#[derive(Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub records: Vec<PatternStat>,
}

impl KnowledgeBase {
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

    pub fn save(&self) {
        let path = PathBuf::from(DB_PATH);
        if let Some(parent) = path.parent() {
            let _ = create_dir_all(parent);
        }
        let mut file = File::create(&path).unwrap();
        let json = serde_json::to_string_pretty(self).unwrap();
        file.write_all(json.as_bytes()).unwrap();
    }

    pub fn insert_stat(&mut self, stat: PatternStat) {
        self.records.push(stat);
        self.save();
    }

    pub fn summarize(&self) {
        println!("[AURORAE++] Base de savoir : {} projets analysés.", self.records.len());
        for r in &self.records {
            println!("→ {}: {} fn / {} struct / {} trait / {} enum", r.project, r.functions, r.structs, r.traits, r.enums);
        }
    }
}
