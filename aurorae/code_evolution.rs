use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use regex::Regex;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use uuid::Uuid;

use crate::brain::{BrainCore, Thought, Intent};
use crate::security_system::SecuritySystem;
use crate::virtual_machine::VirtualMachine;

/// Système d'évolution de code qui permet à AURORAE++ de se modifier et s'améliorer
pub struct CodeEvolution {
    /// Chemin vers le dossier d'inspiration
    inspiration_path: PathBuf,
    /// Chemin vers le code source du système
    source_path: PathBuf,
    /// Base de connaissances dérivée des sources d'inspiration
    knowledge_base: KnowledgeBase,
    /// Historique des modifications
    modification_history: Vec<CodeModification>,
    /// Sécurité du système pour vérifier les modifications
    security: Arc<Mutex<SecuritySystem>>,
    /// Cerveau de l'IA pour les décisions
    brain: Arc<Mutex<BrainCore>>,
    /// Environnement d'exécution sandbox pour tester les modifications
    sandbox: Option<VirtualMachine>,
    /// Règles pour déterminer quelles parties du code peuvent être modifiées
    modification_rules: Vec<ModificationRule>,
    /// Métriques de performance pour évaluer les améliorations
    performance_metrics: PerformanceMetrics,
    /// Stratégies d'évolution par module
    evolution_strategies: HashMap<String, EvolutionStrategy>,
    /// Niveau d'auto-amélioration actuel
    self_improvement_level: u32,
}

/// Base de connaissances extraite des sources d'inspiration
#[derive(Default, Serialize, Deserialize)]
pub struct KnowledgeBase {
    /// Concepts et idées extraits
    pub concepts: HashMap<String, Concept>,
    /// Fragments de code utiles
    pub code_fragments: Vec<CodeFragment>,
    /// Algorithmes découverts
    pub algorithms: Vec<Algorithm>,
    /// Graphe de relations entre concepts
    pub concept_graph: Vec<(String, String, f32)>,
    /// Dernier scan des connaissances
    pub last_update: Option<SystemTime>,
}

/// Représentation d'un concept ou d'une idée
#[derive(Clone, Serialize, Deserialize)]
pub struct Concept {
    pub name: String,
    pub description: String,
    pub relevance: f32,
    pub complexity: f32,
    pub source_files: Vec<String>,
    pub related_concepts: Vec<String>,
}

/// Fragment de code réutilisable
#[derive(Clone, Serialize, Deserialize)]
pub struct CodeFragment {
    pub id: Uuid,
    pub code: String,
    pub language: String,
    pub description: String,
    pub source_file: String,
    pub complexity: f32,
    pub tags: Vec<String>,
    pub performance_score: Option<f32>,
}

/// Algorithme identifié
#[derive(Clone, Serialize, Deserialize)]
pub struct Algorithm {
    pub name: String,
    pub purpose: String,
    pub code_fragments: Vec<Uuid>,
    pub time_complexity: String,
    pub space_complexity: String,
    pub adaptability: f32,
}

/// Historique d'une modification de code
#[derive(Clone, Serialize, Deserialize)]
pub struct CodeModification {
    pub id: Uuid,
    pub timestamp: SystemTime,
    pub target_file: String,
    pub description: String,
    pub changes: String,
    pub purpose: String,
    pub inspiration_sources: Vec<String>,
    pub verification_status: VerificationStatus,
    pub performance_impact: Option<f32>,
}

/// Statut de vérification d'une modification
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum VerificationStatus {
    Pending,
    VerifiedSafe,
    VerifiedImproved,
    Failed,
    Reverted,
}

/// Règle de modification de code
#[derive(Clone, Serialize, Deserialize)]
pub struct ModificationRule {
    pub pattern: String,
    pub allowed_modules: Vec<String>,
    pub risk_level: u8,
    pub require_formal_verification: bool,
    pub require_performance_test: bool,
    pub approval_threshold: f32,
}

/// Métriques de performance
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub execution_time: HashMap<String, Vec<Duration>>,
    pub memory_usage: HashMap<String, Vec<usize>>,
    pub code_complexity: HashMap<String, f32>,
    pub test_coverage: HashMap<String, f32>,
    pub successful_modifications: u32,
    pub failed_modifications: u32,
}

/// Stratégie d'évolution pour un module
#[derive(Clone, Serialize, Deserialize)]
pub struct EvolutionStrategy {
    pub module_name: String,
    pub priority: u8,
    pub focus_areas: Vec<String>,
    pub improvement_targets: HashMap<String, f32>,
    pub success_rate: f32,
    pub algorithm_preferences: Vec<String>,
}

impl CodeEvolution {
    /// Crée une nouvelle instance du système d'évolution de code
    pub fn new(
        inspiration_path: &str, 
        source_path: &str,
        security: Arc<Mutex<SecuritySystem>>,
        brain: Arc<Mutex<BrainCore>>
    ) -> Self {
        let inspiration = PathBuf::from(inspiration_path);
        let source = PathBuf::from(source_path);
        
        println!("[EVOLUTION] 🧬 Initialisation du système d'évolution de code");
        println!("[EVOLUTION] 📂 Dossier d'inspiration: {}", inspiration.display());
        
        let mut evolution = Self {
            inspiration_path: inspiration,
            source_path: source,
            knowledge_base: KnowledgeBase::default(),
            modification_history: Vec::new(),
            security,
            brain,
            sandbox: None,
            modification_rules: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
            evolution_strategies: HashMap::new(),
            self_improvement_level: 1,
        };
        
        // Initialiser les règles de base
        evolution.initialize_modification_rules();
        
        // Initialiser les stratégies d'évolution
        evolution.initialize_evolution_strategies();
        
        evolution
    }
    
    /// Initialise les règles de modification par défaut
    fn initialize_modification_rules(&mut self) {
        let default_rules = vec![
            ModificationRule {
                pattern: r"fn\s+\w+\s*\([^)]*\)\s*(?:->\s*\w+\s*)?\{".to_string(),
                allowed_modules: vec!["optimization".to_string(), "learning".to_string()],
                risk_level: 3,
                require_formal_verification: true,
                require_performance_test: true,
                approval_threshold: 0.8,
            },
            ModificationRule {
                pattern: r"struct\s+\w+\s*\{[^}]*\}".to_string(),
                allowed_modules: vec!["data_structures".to_string()],
                risk_level: 4,
                require_formal_verification: true,
                require_performance_test: false,
                approval_threshold: 0.9,
            },
            ModificationRule {
                pattern: r"impl\s+\w+(?:\s+for\s+\w+)?\s*\{[^}]*\}".to_string(),
                allowed_modules: vec!["optimization".to_string(), "learning".to_string()],
                risk_level: 3,
                require_formal_verification: true,
                require_performance_test: true,
                approval_threshold: 0.85,
            },
            ModificationRule {
                pattern: r"//\s*AURORAE-EVOLVABLE:.*?$".to_string(),
                allowed_modules: vec!["all".to_string()],
                risk_level: 1,
                require_formal_verification: false,
                require_performance_test: true,
                approval_threshold: 0.7,
            },
        ];
        
        self.modification_rules = default_rules;
    }
    
    /// Initialise les stratégies d'évolution par module
    fn initialize_evolution_strategies(&mut self) {
        let strategies = vec![
            EvolutionStrategy {
                module_name: "consensus".to_string(),
                priority: 9,
                focus_areas: vec!["performance".to_string(), "security".to_string()],
                improvement_targets: {
                    let mut targets = HashMap::new();
                    targets.insert("transaction_throughput".to_string(), 0.3);
                    targets.insert("validation_speed".to_string(), 0.4);
                    targets.insert("security_robustness".to_string(), 0.3);
                    targets
                },
                success_rate: 0.0,
                algorithm_preferences: vec!["parallel".to_string(), "cryptographic".to_string()],
            },
            EvolutionStrategy {
                module_name: "learning_agent".to_string(),
                priority: 10,
                focus_areas: vec!["adaptability".to_string(), "meta_learning".to_string()],
                improvement_targets: {
                    let mut targets = HashMap::new();
                    targets.insert("learning_speed".to_string(), 0.4);
                    targets.insert("pattern_recognition".to_string(), 0.3);
                    targets.insert("knowledge_transfer".to_string(), 0.3);
                    targets
                },
                success_rate: 0.0,
                algorithm_preferences: vec!["neural".to_string(), "evolutionary".to_string(), "reinforcement".to_string()],
            },
            EvolutionStrategy {
                module_name: "security_system".to_string(),
                priority: 8,
                focus_areas: vec!["threat_detection".to_string(), "response".to_string()],
                improvement_targets: {
                    let mut targets = HashMap::new();
                    targets.insert("detection_rate".to_string(), 0.5);
                    targets.insert("false_positive_rate".to_string(), 0.2);
                    targets.insert("response_time".to_string(), 0.3);
                    targets
                },
                success_rate: 0.0,
                algorithm_preferences: vec!["heuristic".to_string(), "statistical".to_string()],
            },
        ];
        
        for strategy in strategies {
            self.evolution_strategies.insert(strategy.module_name.clone(), strategy);
        }
    }
    
    /// Analyse le dossier d'inspiration et construit une base de connaissances
    pub fn scan_inspiration_folder(&mut self) -> Result<(), String> {
        println!("[EVOLUTION] 🔍 Scan du dossier d'inspiration en cours...");
        
        if !self.inspiration_path.exists() {
            return Err(format!("Le dossier d'inspiration n'existe pas: {}", self.inspiration_path.display()));
        }
        
        let mut new_knowledge_base = KnowledgeBase::default();
        new_knowledge_base.last_update = Some(SystemTime::now());
        
        // Parcourir tous les fichiers du dossier d'inspiration
        for entry in WalkDir::new(&self.inspiration_path).into_iter().filter_map(Result::ok) {
            if entry.file_type().is_file() {
                let file_path = entry.path();
                
                // Traiter le fichier en fonction de son extension
                if let Some(extension) = file_path.extension() {
                    match extension.to_str().unwrap_or("") {
                        "md" => self.process_markdown_file(file_path, &mut new_knowledge_base)?,
                        "rs" => self.process_rust_file(file_path, &mut new_knowledge_base)?,
                        "py" => self.process_python_file(file_path, &mut new_knowledge_base)?,
                        "json" => self.process_json_file(file_path, &mut new_knowledge_base)?,
                        "txt" => self.process_text_file(file_path, &mut new_knowledge_base)?,
                        _ => { /* Ignorer les autres types de fichiers */ }
                    }
                }
            }
        }
        
        // Analyser les relations entre concepts
        self.analyze_concept_relationships(&mut new_knowledge_base);
        
        // Générer des méta-insights sur les connaissances
        self.generate_meta_insights(&mut new_knowledge_base)?;
        
        // Remplacer l'ancienne base par la nouvelle
        self.knowledge_base = new_knowledge_base;
        
        println!("[EVOLUTION] ✅ Scan terminé! Base de connaissances mise à jour:");
        println!("[EVOLUTION] - {} concepts identifiés", self.knowledge_base.concepts.len());
        println!("[EVOLUTION] - {} fragments de code extraits", self.knowledge_base.code_fragments.len());
        println!("[EVOLUTION] - {} algorithmes reconnus", self.knowledge_base.algorithms.len());
        
        Ok(())
    }
    
    /// Traite un fichier Markdown pour en extraire des concepts
    fn process_markdown_file(&self, file_path: &Path, kb: &mut KnowledgeBase) -> Result<(), String> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Erreur de lecture du fichier Markdown: {}", e))?;
        
        // Extraire les titres comme concepts
        let heading_regex = Regex::new(r"#+\s+(.+)").unwrap();
        for cap in heading_regex.captures_iter(&content) {
            let concept_name = cap[1].to_string();
            
            // Extraire le contexte autour du titre
            let pos = cap.get(0).unwrap().start();
            let context_start = content[..pos].rfind("\n\n").unwrap_or(0);
            let context_end = content[pos..].find("\n\n").map_or(content.len(), |p| pos + p);
            let description = content[context_start..context_end].trim().to_string();
            
            // Créer ou mettre à jour le concept
            let concept = Concept {
                name: concept_name.clone(),
                description,
                relevance: 0.8, // Valeur initiale
                complexity: 0.5, // Valeur initiale
                source_files: vec![file_path.to_string_lossy().to_string()],
                related_concepts: Vec::new(),
            };
            
            kb.concepts.insert(concept_name, concept);
        }
        
        // Extraire les blocs de code
        let code_regex = Regex::new(r"```(\w*)\n([\s\S]*?)\n```").unwrap();
        for cap in code_regex.captures_iter(&content) {
            let language = cap[1].to_string();
            let code = cap[2].to_string();
            
            if code.trim().is_empty() {
                continue;
            }
            
            // Extraire une description du code (texte précédant le bloc)
            let block_start = cap.get(0).unwrap().start();
            let desc_start = content[..block_start].rfind("\n\n").unwrap_or(0);
            let description = content[desc_start..block_start].trim().to_string();
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code,
                language: if language.is_empty() { "text".to_string() } else { language },
                description,
                source_file: file_path.to_string_lossy().to_string(),
                complexity: 0.5, // Valeur initiale
                tags: Vec::new(),
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
        }
        
        Ok(())
    }
    
    /// Traite un fichier Rust pour en extraire des fragments de code et des algorithmes
    fn process_rust_file(&self, file_path: &Path, kb: &mut KnowledgeBase) -> Result<(), String> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Erreur de lecture du fichier Rust: {}", e))?;
        
        // Extraire les structures
        let struct_regex = Regex::new(r"struct\s+(\w+)(?:<[^>]*>)?\s*\{([\s\S]*?)\}").unwrap();
        for cap in struct_regex.captures_iter(&content) {
            let struct_name = cap[1].to_string();
            let struct_body = cap[2].to_string();
            
            // Extraire les commentaires de documentation
            let doc_regex = Regex::new(r"///\s*(.+)").unwrap();
            let mut description = String::new();
            for doc in doc_regex.captures_iter(&content[..cap.get(0).unwrap().start()]) {
                description.push_str(&doc[1]);
                description.push('\n');
            }
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code: format!("struct {} {{\n{}\n}}", struct_name, struct_body),
                language: "rust".to_string(),
                description: if description.is_empty() { format!("Structure {}", struct_name) } else { description },
                source_file: file_path.to_string_lossy().to_string(),
                complexity: struct_body.lines().count() as f32 / 10.0,
                tags: vec!["struct".to_string(), struct_name.clone()],
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
            
            // Ajouter comme concept
            if !kb.concepts.contains_key(&struct_name) {
                let concept = Concept {
                    name: struct_name.clone(),
                    description: format!("Structure de données Rust '{}'", struct_name),
                    relevance: 0.7,
                    complexity: struct_body.lines().count() as f32 / 20.0,
                    source_files: vec![file_path.to_string_lossy().to_string()],
                    related_concepts: Vec::new(),
                };
                
                kb.concepts.insert(struct_name, concept);
            }
        }
        
        // Extraire les implémentations
        let impl_regex = Regex::new(r"impl(?:<[^>]*>)?\s+(\w+)(?:\s+for\s+(\w+))?\s*\{([\s\S]*?)\}").unwrap();
        for cap in impl_regex.captures_iter(&content) {
            let impl_name = if let Some(for_type) = cap.get(2) {
                format!("{} for {}", cap[1].to_string(), for_type.as_str())
            } else {
                cap[1].to_string()
            };
            
            let impl_body = cap[3].to_string();
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code: format!("impl {} {{\n{}\n}}", impl_name, impl_body),
                language: "rust".to_string(),
                description: format!("Implémentation pour {}", impl_name),
                source_file: file_path.to_string_lossy().to_string(),
                complexity: impl_body.lines().count() as f32 / 15.0,
                tags: vec!["impl".to_string(), impl_name.clone()],
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
        }
        
        // Extraire les fonctions
        let fn_regex = Regex::new(r"fn\s+(\w+)(?:<[^>]*>)?\s*\(([^)]*)\)(?:\s*->\s*([^{]+))?\s*\{([\s\S]*?)(?:^\}|[^\S\r\n]\})").unwrap();
        for cap in fn_regex.captures_iter(&content) {
            let fn_name = cap[1].to_string();
            let fn_params = cap[2].to_string();
            let fn_return = cap.get(3).map_or("".to_string(), |m| m.as_str().to_string());
            let fn_body = cap[4].to_string();
            
            // Extraire la description de la fonction depuis les commentaires
            let start_pos = cap.get(0).unwrap().start();
            let preceding = &content[..start_pos];
            let doc_start = preceding.rfind("///").unwrap_or(preceding.len());
            let mut description = String::new();
            
            let doc_regex = Regex::new(r"///\s*(.+)").unwrap();
            for doc in doc_regex.captures_iter(&preceding[doc_start..]) {
                description.push_str(&doc[1]);
                description.push('\n');
            }
            
            let fn_code = format!("fn {}({}){}{{ {} }}", 
                fn_name, fn_params, 
                if fn_return.is_empty() { " ".to_string() } else { format!(" -> {} ", fn_return) },
                fn_body);
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code: fn_code.clone(),
                language: "rust".to_string(),
                description: if description.is_empty() { format!("Fonction {}", fn_name) } else { description },
                source_file: file_path.to_string_lossy().to_string(),
                complexity: fn_body.lines().count() as f32 / 10.0,
                tags: vec!["function".to_string(), fn_name.clone()],
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
            
            // Détecter si c'est un algorithme
            if fn_body.contains("for") || fn_body.contains("while") || fn_body.contains("recursion") {
                let algorithm = Algorithm {
                    name: fn_name.clone(),
                    purpose: format!("Fonction extraite de {}", file_path.file_name().unwrap().to_string_lossy()),
                    code_fragments: vec![fragment.id],
                    time_complexity: "O(n)".to_string(), // Estimation par défaut
                    space_complexity: "O(1)".to_string(), // Estimation par défaut
                    adaptability: 0.7,
                };
                
                kb.algorithms.push(algorithm);
            }
        }
        
        Ok(())
    }
    
    /// Traite un fichier Python pour en extraire des fragments de code et des algorithmes
    fn process_python_file(&self, file_path: &Path, kb: &mut KnowledgeBase) -> Result<(), String> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Erreur de lecture du fichier Python: {}", e))?;
        
        // Extraire les classes
        let class_regex = Regex::new(r"class\s+(\w+)(?:\(([^)]*)\))?\s*:([\s\S]*?)(?:^\S|\Z)").unwrap();
        for cap in class_regex.captures_iter(&content) {
            let class_name = cap[1].to_string();
            let class_body = cap[3].to_string();
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code: format!("class {}:\n{}", class_name, class_body),
                language: "python".to_string(),
                description: format!("Classe Python '{}'", class_name),
                source_file: file_path.to_string_lossy().to_string(),
                complexity: class_body.lines().count() as f32 / 10.0,
                tags: vec!["class".to_string(), class_name.clone()],
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
        }
        
        // Extraire les fonctions
        let fn_regex = Regex::new(r"def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?\s*:([\s\S]*?)(?:^\S|\Z)").unwrap();
        for cap in fn_regex.captures_iter(&content) {
            let fn_name = cap[1].to_string();
            let fn_params = cap[2].to_string();
            let fn_body = cap[4].to_string();
            
            let fragment = CodeFragment {
                id: Uuid::new_v4(),
                code: format!("def {}({}):\n{}", fn_name, fn_params, fn_body),
                language: "python".to_string(),
                description: format!("Fonction Python '{}'", fn_name),
                source_file: file_path.to_string_lossy().to_string(),
                complexity: fn_body.lines().count() as f32 / 8.0,
                tags: vec!["function".to_string(), fn_name.clone()],
                performance_score: None,
            };
            
            kb.code_fragments.push(fragment);
            
            // Détecter si c'est un algorithme
            if fn_body.contains("for") || fn_body.contains("while") || fn_body.contains("recursion") {
                let algorithm = Algorithm {
                    name: fn_name.clone(),
                    purpose: format!("Fonction extraite de {}", file_path.file_name().unwrap().to_string_lossy()),
                    code_fragments: vec![fragment.id],
                    time_complexity: "O(n)".to_string(), // Estimation par défaut
                    space_complexity: "O(1)".to_string(), // Estimation par défaut
                    adaptability: 0.6,
                };
                
                kb.algorithms.push(algorithm);
            }
        }
        
        Ok(())
    }
    
    /// Traite un fichier JSON pour en extraire des données structurées
    fn process_json_file(&self, file_path: &Path, kb: &mut KnowledgeBase) -> Result<(), String> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Erreur de lecture du fichier JSON: {}", e))?;
        
        // Tenter de parser le JSON
        match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(json) => {
                // Ajouter le fichier JSON comme fragment de code
                let fragment = CodeFragment {
                    id: Uuid::new_v4(),
                    code: content,
                    language: "json".to_string(),
                    description: format!("Données JSON de {}", file_path.file_name().unwrap().to_string_lossy()),
                    source_file: file_path.to_string_lossy().to_string(),
                    complexity: 0.3,
                    tags: vec!["data".to_string(), "json".to_string()],
                    performance_score: None,
                };
                
                kb.code_fragments.push(fragment);
                
                // Extraire les clés de premier niveau comme concepts
                if let serde_json::Value::Object(map) = json {
                    for (key, value) in map {
                        if !kb.concepts.contains_key(&key) {
                            let description = match value {
                                serde_json::Value::String(s) => s,
                                _ => format!("Concept JSON extrait de {}", file_path.file_name().unwrap().to_string_lossy()),
                            };
                            
                            let concept = Concept {
                                name: key.clone(),
                                description,
                                relevance: 0.6,
                                complexity: 0.3,
                                source_files: vec![file_path.to_string_lossy().to_string()],
                                related_concepts: Vec::new(),
                            };
                            
                            kb.concepts.insert(key, concept);
                        }
                    }
                }
            },
            Err(_) => {
                // Si le parsing échoue, juste ignorer ce fichier
                return Ok(());
            }
        }
        
        Ok(())
    }
    
    /// Traite un fichier texte pour en extraire des concepts
    fn process_text_file(&self, file_path: &Path, kb: &mut KnowledgeBase) -> Result<(), String> {
        let mut content = String::new();
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut content))
            .map_err(|e| format!("Erreur de lecture du fichier texte: {}", e))?;
        
        // Ajouter le contenu comme un fragment
        let fragment = CodeFragment {
            id: Uuid::new_v4(),
            code: content.clone(),
            language: "text".to_string(),
            description: format!("Texte de {}", file_path.file_name().unwrap().to_string_lossy()),
            source_file: file_path.to_string_lossy().to_string(),
            complexity: 0.1,
            tags: vec!["text".to_string()],
            performance_score: None,
        };
        
        kb.code_fragments.push(fragment);
        
        // Extraire des mots-clés potentiels (mots en majuscules ou phrases entre guillemets)
        let keyword_regex = Regex::new(r#"([A-Z][A-Z_]+|"([^"]+)")"#).unwrap();
        for cap in keyword_regex.captures_iter(&content) {
            let keyword = cap.get(2).map_or_else(|| cap[1].to_string(), |m| m.as_str().to_string());
            
            if keyword.len() > 3 && !kb.concepts.contains_key(&keyword) {
                // Extraire le contexte autour du mot-clé
                let pos = cap.get(0).unwrap().start();
                let context_start = content[..pos].rfind(".").unwrap_or(0);
                let context_end = content[pos..].find(".").map_or(content.len(), |p| pos + p + 1);
                let description = content[context_start..context_end].trim().to_string();
                
                let concept = Concept {
                    name: keyword.clone(),
                    description,
                    relevance: 0.5,
                    complexity: 0.2,
                    source_files: vec![file_path.to_string_lossy().to_string()],
                    related_concepts: Vec::new(),
                };
                
                kb.concepts.insert(keyword, concept);
            }
        }
        
        Ok(())
    }
    
    /// Analyse les relations entre les concepts
    fn analyze_concept_relationships(&self, kb: &mut KnowledgeBase) {
        let mut relationships = Vec::new();
        let concept_names: Vec<String> = kb.concepts.keys().cloned().collect();
        
        // Construire un graphe de relations entre concepts
        for i in 0..concept_names.len() {
            let concept_i = &concept_names[i];
            let concept_i_data = kb.concepts.get(concept_i).unwrap();
            
            for j in (i+1)..concept_names.len() {
                let concept_j = &concept_names[j];
                let concept_j_data = kb.concepts.get(concept_j).unwrap();
                
                // Calcul de similarité simple: nombres de fichiers sources communs
                let mut common_sources = 0;
                for source_i in &concept_i_data.source_files {
                    if concept_j_data.source_files.contains(source_i) {
                        common_sources += 1;
                    }
                }
                
                // Si présent dans la description
                let desc_relation = if concept_i_data.description.contains(concept_j) {
                    0.3
                } else if concept_j_data.description.contains(concept_i) {
                    0.3
                } else {
                    0.0
                };
                
                let source_relation = if common_sources > 0 {
                    0.5 * (common_sources as f32 / concept_i_data.source_files.len().max(1) as f32)
                } else {
                    0.0
                };
                
                let relation_strength = desc_relation + source_relation;
                
                if relation_strength > 0.2 {
                    relationships.push((concept_i.clone(), concept_j.clone(), relation_strength));
                    
                    // Mettre à jour les concepts liés
                    if let Some(concept) = kb.concepts.get_mut(concept_i) {
                        if !concept.related_concepts.contains(concept_j) {
                            concept.related_concepts.push(concept_j.clone());
                        }
                    }
                    
                    if let Some(concept) = kb.concepts.get_mut(concept_j) {
                        if !concept.related_concepts.contains(concept_i) {
                            concept.related_concepts.push(concept_i.clone());
                        }
                    }
                }
            }
        }
        
        kb.concept_graph = relationships;
    }
    
    /// Génère des méta-insights basés sur la base de connaissances
    fn generate_meta_insights(&self, kb: &mut KnowledgeBase) -> Result<(), String> {
        // Identifier les concepts les plus connectés (centraux)
        let mut concept_connections: HashMap<String, usize> = HashMap::new();
        
        for (src, dst, _) in &kb.concept_graph {
            *concept_connections.entry(src.clone()).or_insert(0) += 1;
            *concept_connections.entry(dst.clone()).or_insert(0) += 1;
        }
        
        // Mettre à jour la pertinence des concepts basée sur leur centralité
        for (concept_name, connections) in concept_connections {
            if let Some(concept) = kb.concepts.get_mut(&concept_name) {
                concept.relevance = (0.5 + (connections as f32 * 0.1)).min(1.0);
            }
        }
        
        // Identifier les algorithmes potentiellement utiles qui ne sont pas encore dans la base
        let mut code_by_language: HashMap<String, Vec<&CodeFragment>> = HashMap::new();
        
        for fragment in &kb.code_fragments {
            code_by_language.entry(fragment.language.clone())
                .or_insert_with(Vec::new)
                .push(fragment);
        }
        
        // Pour chaque langage, rechercher des modèles algorithmiques
        for (language, fragments) in &code_by_language {
            match language.as_str() {
                "rust" => self.identify_rust_algorithms(fragments, kb)?,
                "python" => self.identify_python_algorithms(fragments, kb)?,
                _ => {}
            }
        }
        
        Ok(())
    }
    
    /// Identifie des algorithmes Rust à partir de fragments de code
    fn identify_rust_algorithms(&self, fragments: &[&CodeFragment], kb: &mut KnowledgeBase) -> Result<(), String> {
        // Patterns pour des algorithmes courants en Rust
        let patterns = vec![
            (r"for\s+.*\s+in\s+.*\.iter\(\).*", "Itération", "O(n)", "O(1)"),
            (r"\.fold\(.*\)", "Réduction/Agrégation", "O(n)", "O(1)"),
            (r"\.map\(.*\).*\.filter\(.*\)", "Transformation de données", "O(n)", "O(n)"),
            (r"\.sort_by\(.*\)|\.sort\(\)", "Tri", "O(n log n)", "O(1)"),
            (r"let\s+mut\s+.*\s*=\s*HashMap::new\(\);", "Table de hachage", "O(1) moyenne", "O(n)"),
            (r"\.binary_search\(.*\)", "Recherche binaire", "O(log n)", "O(1)"),
            (r"fn\s+.*\(.*\).*\{.*\s+if\s+.*\s+{\s+.*\s+}\s+else\s+{\s+.*\s+}\s+.*\}", "Décision conditionnelle", "O(1)", "O(1)"),
            (r"fn\s+.*\(.*\).*\{.*\s+match\s+.*\s+{\s+.*\s+}\s+.*\}", "Pattern matching", "O(1)", "O(1)"),
            (r"fn\s+.*\(.*\).*\{.*\s+.*\(.*\).*\s+.*\}", "Récursion", "Varie", "O(n)"),
            (r"async\s+fn|\.await", "Asynchrone", "Varie", "Varie"),
            (r"parallel|rayon", "Parallélisme", "O(n/p)", "O(n)"),
        ];
        
        // Pour chaque fragment, rechercher des patterns algorithmiques
        for fragment in fragments {
            let mut matched_algorithms = Vec::new();
            
            for (pattern, name, time, space) in &patterns {
                let regex = match Regex::new(pattern) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                
                if regex.is_match(&fragment.code) {
                    matched_algorithms.push((name, time, space));
                }
            }
            
            for (algo_name, time, space) in matched_algorithms {
                // Vérifier si cet algorithme est déjà identifié
                let algo_exists = kb.algorithms.iter().any(|a| a.name == *algo_name && 
                                                            a.code_fragments.contains(&fragment.id));
                
                if !algo_exists {
                    let algorithm = Algorithm {
                        name: format!("{} ({})", algo_name, fragment.tags.first().unwrap_or(&"inconnu".to_string())),
                        purpose: fragment.description.clone(),
                        code_fragments: vec![fragment.id],
                        time_complexity: (*time).to_string(),
                        space_complexity: (*space).to_string(),
                        adaptability: 0.7,
                    };
                    
                    kb.algorithms.push(algorithm);
                }
            }
        }
        
        Ok(())
    }
    
    /// Identifie des algorithmes Python à partir de fragments de code
    fn identify_python_algorithms(&self, fragments: &[&CodeFragment], kb: &mut KnowledgeBase) -> Result<(), String> {
        // Patterns pour des algorithmes courants en Python
        let patterns = vec![
            (r"for\s+.*\s+in\s+.*:", "Itération", "O(n)", "O(1)"),
            (r"reduce\(.*\)|sum\(.*\)", "Réduction/Agrégation", "O(n)", "O(1)"),
            (r"map\(.*\)|filter\(.*\)|list\s+comprehension", "Transformation de données", "O(n)", "O(n)"),
            (r"sorted\(.*\)|\.sort\(\)", "Tri", "O(n log n)", "O(1)"),
            (r"dict\(.*\)|{.*:.*}", "Table de hachage", "O(1) moyenne", "O(n)"),
            (r"binary_search|bisect", "Recherche binaire", "O(log n)", "O(1)"),
            (r"if\s+.*\s*:\s*.*\s*else\s*:", "Décision conditionnelle", "O(1)", "O(1)"),
            (r"def\s+.*\(.*\).*:\s*.*\s+return\s+.*\(.*\)", "Récursion", "Varie", "O(n)"),
            (r"async\s+def|await", "Asynchrone", "Varie", "Varie"),
            (r"parallel|multiprocessing|concurrent", "Parallélisme", "O(n/p)", "O(n)"),
            (r"@lru_cache|memoize", "Mémoïsation", "Amélioré", "O(n)"),
        ];
        
        // Pour chaque fragment, rechercher des patterns algorithmiques
        for fragment in fragments {
            let mut matched_algorithms = Vec::new();
            
            for (pattern, name, time, space) in &patterns {
                let regex = match Regex::new(pattern) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                
                if regex.is_match(&fragment.code) {
                    matched_algorithms.push((name, time, space));
                }
            }
            
            for (algo_name, time, space) in matched_algorithms {
                // Vérifier si cet algorithme est déjà identifié
                let algo_exists = kb.algorithms.iter().any(|a| a.name == *algo_name && 
                                                           a.code_fragments.contains(&fragment.id));
                
                if !algo_exists {
                    let algorithm = Algorithm {
                        name: (*algo_name).to_string(),
                        purpose: fragment.description.clone(),
                        code_fragments: vec![fragment.id],
                        time_complexity: (*time).to_string(),
                        space_complexity: (*space).to_string(),
                        adaptability: 0.65,
                    };
                    
                    kb.algorithms.push(algorithm);
                }
            }
        }
        
        Ok(())
    }
    
    /// Identifie les améliorations potentielles dans le code source
    pub fn identify_improvement_opportunities(&self) -> Vec<ImprovementOpportunity> {
        println!("[EVOLUTION] 🔍 Recherche d'opportunités d'amélioration dans le code...");
        
        let mut opportunities = Vec::new();
        
        // Parcourir les fichiers source
        for entry in WalkDir::new(&self.source_path).into_iter().filter_map(Result::ok) {
            if !entry.file_type().is_file() {
                continue;
            }
            
            let file_path = entry.path();
            
            // Ne traiter que les fichiers Rust
            if let Some(ext) = file_path.extension() {
                if ext != "rs" {
                    continue;
                }
            } else {
                continue;
            }
            
            // Lire le contenu du fichier
            let content = match fs::read_to_string(file_path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            
            // Rechercher les sections marquées comme évolutives
            let evolvable_regex = Regex::new(r"//\s*AURORAE-EVOLVABLE:\s*(.+)").unwrap();
            for cap in evolvable_regex.captures_iter(&content) {
                let description = cap[1].to_string();
                let pos = cap.get(0).unwrap().start();
                
                // Trouver la prochaine fonction ou structure après ce commentaire
                let fn_regex = Regex::new(r"fn\s+(\w+)").unwrap();
                let struct_regex = Regex::new(r"struct\s+(\w+)").unwrap();
                
                let section = &content[pos..];
                let target_name = if let Some(fn_cap) = fn_regex.captures(section) {
                    fn_cap[1].to_string()
                } else if let Some(struct_cap) = struct_regex.captures(section) {
                    struct_cap[1].to_string()
                } else {
                    continue;
                };
                
                // Extraire la section de code concernée
                let end_pos = section.find("\n}").map_or(section.len(), |p| pos + p + 2);
                let code_section = content[pos..end_pos].to_string();
                
                // Trouver des algorithmes pertinents dans la base de connaissances
                let mut relevant_algorithms = Vec::new();
                for algorithm in &self.knowledge_base.algorithms {
                    for &fragment_id in &algorithm.code_fragments {
                        if let Some(fragment) = self.knowledge_base.code_fragments.iter().find(|f| f.id == fragment_id) {
                            if fragment.code.contains(&target_name) || 
                               fragment.tags.iter().any(|t| target_name.contains(t)) {
                                relevant_algorithms.push(algorithm.clone());
                                break;
                            }
                        }
                    }
                }
                
                // Créer une opportunité d'amélioration
                let opportunity = ImprovementOpportunity {
                    file_path: file_path.to_string_lossy().to_string(),
                    target_name,
                    description,
                    current_code: code_section,
                    suggested_algorithms: relevant_algorithms,
                    improvement_score: 0.7, // Score initial
                };
                
                opportunities.push(opportunity);
            }
            
            // Rechercher des opportunités d'amélioration basées sur des motifs connus
            self.find_performance_improvements(&content, file_path, &mut opportunities);
            self.find_security_improvements(&content, file_path, &mut opportunities);
            self.find_code_quality_improvements(&content, file_path, &mut opportunities);
        }
        
        // Trier les opportunités par score d'amélioration
        opportunities.sort_by(|a, b| b.improvement_score.partial_cmp(&a.improvement_score).unwrap());
        
        println!("[EVOLUTION] ✅ {} opportunités d'amélioration identifiées", opportunities.len());
        
        opportunities
    }
    
    /// Recherche des opportunités d'amélioration de performance
    fn find_performance_improvements(&self, content: &str, file_path: &Path, opportunities: &mut Vec<ImprovementOpportunity>) {
        // Patterns pour les problèmes de performance courants
        let patterns = vec![
            (r"for\s+.*\s+in\s+.*\.clone\(\)", "Clonage inutile dans une boucle", 0.8),
            (r"let\s+mut\s+.*\s*=\s*Vec::new\(\);\s+for\s+.*\s+{\s+.*\.push\(.*\);\s+}", "Utiliser un constructeur de collection au lieu de push répétés", 0.7),
            (r"\.to_string\(\).*\.to_string\(\)", "Conversions de chaînes multiples", 0.6),
            (r"for\s+i\s+in\s+0\.\..*.len\(\)\s+{.*\[i\]", "Utiliser une itération directe plutôt que des indices", 0.7),
            (r"if\s+let\s+Some\(.*\)\s+=\s+.*\s+{\s+.*\s+}\s+if\s+let\s+Some\(.*\)\s+=\s+.*\s+{", "Combiner des if let multiples", 0.5),
        ];
        
        for (pattern, description, score) in patterns {
            let regex = match Regex::new(pattern) {
                Ok(r) => r,
                Err(_) => continue,
            };
            
            for cap in regex.captures_iter(content) {
                let matched_code = cap[0].to_string();
                let pos = cap.get(0).unwrap().start();
                
                // Trouver la fonction contenant ce code
                let fn_start = content[..pos].rfind("fn ").unwrap_or(0);
                let fn_regex = Regex::new(r"fn\s+(\w+)").unwrap();
                
                if let Some(fn_cap) = fn_regex.captures(&content[fn_start..]) {
                    let fn_name = fn_cap[1].to_string();
                    
                    // Créer une opportunité d'amélioration
                    let opportunity = ImprovementOpportunity {
                        file_path: file_path.to_string_lossy().to_string(),
                        target_name: fn_name,
                        description: format!("Amélioration de performance: {}", description),
                        current_code: matched_code,
                        suggested_algorithms: Vec::new(),
                        improvement_score: score,
                    };
                    
                    opportunities.push(opportunity);
                }
            }
        }
    }
    
    /// Recherche des opportunités d'amélioration de sécurité
    fn find_security_improvements(&self, content: &str, file_path: &Path, opportunities: &mut Vec<ImprovementOpportunity>) {
        // Patterns pour les problèmes de sécurité courants
        let patterns = vec![
            (r"unsafe\s+{", "Bloc unsafe non protégé", 0.9),
            (r"let\s+.*\s*=\s*String::from\(.*input.*\)", "Entrée utilisateur non validée", 0.85),
            (r"\.unwrap\(\)", "Gestion d'erreur avec unwrap()", 0.7),
            (r"panic!\(", "Utilisation de panic!", 0.6),
            (r"std::mem::transmute", "Utilisation de transmute", 0.95),
        ];
        
        for (pattern, description, score) in patterns {
            let regex = match Regex::new(pattern) {
                Ok(r) => r,
                Err(_) => continue,
            };
            
            for cap in regex.captures_iter(content) {
                let matched_code = cap[0].to_string();
                let pos = cap.get(0).unwrap().start();
                
                // Trouver la fonction contenant ce code
                let fn_start = content[..pos].rfind("fn ").unwrap_or(0);
                let fn_regex = Regex::new(r"fn\s+(\w+)").unwrap();
                
                if let Some(fn_cap) = fn_regex.captures(&content[fn_start..]) {
                    let fn_name = fn_cap[1].to_string();
                    
                    // Créer une opportunité d'amélioration
                    let opportunity = ImprovementOpportunity {
                        file_path: file_path.to_string_lossy().to_string(),
                        target_name: fn_name,
                        description: format!("Amélioration de sécurité: {}", description),
                        current_code: matched_code,
                        suggested_algorithms: Vec::new(),
                        improvement_score: score,
                    };
                    
                    opportunities.push(opportunity);
                }
            }
        }
    }
    
    /// Recherche des opportunités d'amélioration de qualité de code
    fn find_code_quality_improvements(&self, content: &str, file_path: &Path, opportunities: &mut Vec<ImprovementOpportunity>) {
        // Patterns pour les problèmes de qualité de code courants
        let patterns = vec![
            (r"fn\s+\w+[^{]*\{[^}]{500,}\}", "Fonction trop longue", 0.8),
            (r"//\s*TODO|//\s*FIXME", "TODO ou FIXME non résolu", 0.6),
            (r"if\s+.*\s+{\s+.*\s+}\s+else\s+if\s+.*\s+{\s+.*\s+}\s+else\s+if\s+.*\s+{\s+.*\s+}\s+else\s+if", "Cascade if-else-if trop longue", 0.7),
            (r"match\s+.*\s+{\s+.*_\s+=>\s+.*,", "Match avec clause catch-all", 0.5),
            (r"\s{4,}//", "Commentaire mal aligné", 0.4),
        ];
        
        for (pattern, description, score) in patterns {
            let regex = match Regex::new(pattern) {
                Ok(r) => r,
                Err(_) => continue,
            };
            
            for cap in regex.captures_iter(content) {
                let matched_code = cap[0].to_string();
                let pos = cap.get(0).unwrap().start();
                
                // Trouver la fonction ou structure contenant ce code
                let fn_start = content[..pos].rfind("fn ").unwrap_or(content[..pos].rfind("struct ").unwrap_or(0));
                let target_regex = Regex::new(r"(fn|struct)\s+(\w+)").unwrap();
                
                if let Some(target_cap) = target_regex.captures(&content[fn_start..]) {
                    let target_type = target_cap[1].to_string();
                    let target_name = target_cap[2].to_string();
                    
                    // Créer une opportunité d'amélioration
                    let opportunity = ImprovementOpportunity {
                        file_path: file_path.to_string_lossy().to_string(),
                        target_name,
                        description: format!("Amélioration de qualité de code ({}) : {}", target_type, description),
                        current_code: matched_code,
                        suggested_algorithms: Vec::new(),
                        improvement_score: score,
                    };
                    
                    opportunities.push(opportunity);
                }
            }
        }
    }
    
    /// Génère des améliorations pour le code basées sur la base de connaissances
    pub fn generate_code_improvements(&self, opportunities: &[ImprovementOpportunity]) 
        -> Result<Vec<CodeImprovement>, String> {
        println!("[EVOLUTION] 🧪 Génération d'améliorations de code...");
        
        let mut improvements = Vec::new();
        
        for opportunity in opportunities {
            println!("[EVOLUTION] - Amélioration pour {}: {}", 
                     opportunity.target_name, opportunity.description);
            
            // Trouver des fragments de code pertinents
            let mut relevant_fragments = Vec::new();
            
            // Chercher des fragments de code en Rust similaires au problème
            for fragment in &self.knowledge_base.code_fragments {
                if fragment.language != "rust" {
                    continue;
                }
                
                let mut relevance = 0.0;
                
                // Vérifier si les tags correspondent
                for tag in &fragment.tags {
                    if opportunity.target_name.contains(tag) || 
                       opportunity.description.contains(tag) {
                        relevance += 0.3;
                    }
                }
                
                // Vérifier la similarité de code
                if fragment.code.contains(&opportunity.current_code) || 
                   opportunity.current_code.contains(&fragment.code) {
                    relevance += 0.5;
                }
                
                if relevance > 0.3 {
                    relevant_fragments.push((fragment, relevance));
                }
            }
            
            // Trier par pertinence
            relevant_fragments.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
            
            // Générer l'amélioration
            let improvement = match opportunity.description.to_lowercase() {
                d if d.contains("performance") => {
                    self.generate_performance_improvement(opportunity, &relevant_fragments)
                },
                d if d.contains("sécurité") => {
                    self.generate_security_improvement(opportunity, &relevant_fragments)
                },
                d if d.contains("qualité") => {
                    self.generate_quality_improvement(opportunity, &relevant_fragments)
                },
                _ => {
                    self.generate_general_improvement(opportunity, &relevant_fragments)
                }
            };
            
            improvements.push(improvement);
        }
        
        println!("[EVOLUTION] ✅ {} améliorations générées", improvements.len());
        
        Ok(improvements)
    }
    
    /// Génère une amélioration de performance
    fn generate_performance_improvement(&self, opportunity: &ImprovementOpportunity, 
                                      relevant_fragments: &[(
