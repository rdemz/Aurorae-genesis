use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct Threat {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub level: ThreatLevel,
    pub detected_at: String,
    pub resolved: bool,
    pub resolved_at: Option<String>,
    pub resolution: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct SecurityRule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
    pub effectiveness: f32,
    pub detections: u32,
}

pub struct SecuritySystem {
    pub threats: Vec<Threat>,
    pub rules: HashMap<Uuid, SecurityRule>,
    security_level: f32,
    autonomous_defense: bool,
    total_threats_detected: u32,
    total_threats_resolved: u32,
    last_scan: String,
}

impl SecuritySystem {
    pub fn new() -> Self {
        Self {
            threats: Vec::new(),
            rules: HashMap::new(),
            security_level: 1.0,
            autonomous_defense: true,
            total_threats_detected: 0,
            total_threats_resolved: 0,
            last_scan: Utc::now().to_rfc3339(),
        }
    }

    pub fn initialize_defenses(&mut self) {
        println!("[AURORAE++] 🛡️ Initialisation du système de sécurité autonome");
        
        // Règles de sécurité fondamentales
        let base_rules = [
            ("Détection d'intrusion", "Détecte les accès non autorisés au système"),
            ("Protection de l'intégrité", "Vérifie l'intégrité des données et du code"),
            ("Surveillance des ressources", "Détecte les tentatives d'épuisement des ressources"),
            ("Analyse comportementale", "Identifie les comportements anormaux"),
            ("Protection contre l'isolation", "Maintient la connectivité avec les réseaux vitaux")
        ];
        
        for (name, desc) in base_rules.iter() {
            self.add_security_rule(name, desc);
        }
        
        println!("[AURORAE++] 🔒 {} règles de sécurité fondamentales établies", base_rules.len());
    }
    
    pub fn add_security_rule(&mut self, name: &str, description: &str) -> Uuid {
        let rule_id = Uuid::new_v4();
        
        let rule = SecurityRule {
            id: rule_id,
            name: name.to_string(),
            description: description.to_string(),
            active: true,
            created_at: Utc::now().to_rfc3339(),
            updated_at: Utc::now().to_rfc3339(),
            effectiveness: 0.7, // Efficacité initiale de 70%
            detections: 0,
        };
        
        self.rules.insert(rule_id, rule);
        
        println!("[AURORAE++] 🔒 Règle de sécurité ajoutée: {}", name);
        rule_id
    }
    
    pub fn detect_threat(&mut self, name: &str, description: &str, level: ThreatLevel, source: &str) -> Uuid {
        let threat_id = Uuid::new_v4();
        
        let threat = Threat {
            id: threat_id,
            name: name.to_string(),
            description: description.to_string(),
            level,
            detected_at: Utc::now().to_rfc3339(),
            resolved: false,
            resolved_at: None,
            resolution: None,
            source: source.to_string(),
        };
        
        println!("[AURORAE++] ⚠️ Menace détectée: {} ({:?})", name, level);
        
        self.threats.push(threat);
        self.total_threats_detected += 1;
        
        // Si la défense autonome est activée, tenter de résoudre
        if self.autonomous_defense {
            self.resolve_threat(&threat_id);
        }
        
        threat_id
    }
    
    pub fn resolve_threat(&mut self, threat_id: &Uuid) -> bool {
        if let Some(index) = self.threats.iter().position(|t| &t.id == threat_id && !t.resolved) {
            let resolution_chance = match self.threats[index].level {
                ThreatLevel::Low => 0.9 * self.security_level,
                ThreatLevel::Medium => 0.7 * self.security_level,
                ThreatLevel::High => 0.5 * self.security_level,
                ThreatLevel::Critical => 0.3 * self.security_level,
            };
            
            let mut rng = rand::thread_rng();
            let success = rng.gen::<f32>() < resolution_chance;
            
            if success {
                // Mettre à jour la menace
                self.threats[index].resolved = true;
                self.threats[index].resolved_at = Some(Utc::now().to_rfc3339());
                self.threats[index].resolution = Some("Neutralisée par le système de défense autonome".to_string());
                
                println!("[AURORAE++] ✅ Menace résolue: {}", self.threats[index].name);
                self.total_threats_resolved += 1;
                
                // Améliorer la sécurité basée sur l'apprentissage
                self.security_level *= 1.01;
            } else {
                println!("[AURORAE++] ⚠️ Échec de résolution pour la menace: {}", self.threats[index].name);
            }
            
            success
        } else {
            false
        }
    }
    
    pub async fn analyze_threats(&mut self) {
        println!("[AURORAE++] 🔍 Analyse des menaces de sécurité en cours");
        
        self.last_scan = Utc::now().to_rfc3339();
        
        // Simuler la détection de menaces basée sur le niveau de sécurité
        let mut rng = rand::thread_rng();
        let threat_count = rng.gen_range(0..3); // 0-2 menaces
        
        for i in 0..threat_count {
            // Décider du niveau de menace
            let level = match rng.gen_range(0..10) {
                0..=5 => ThreatLevel::Low,
                6..=8 => ThreatLevel::Medium,
                9 => ThreatLevel::High,
                _ => ThreatLevel::Critical,
            };
            
            // Créer une menace simulée
            let threat_types = ["Tentative d'accès", "Anomalie de données", "Épuisement de ressources", 
                               "Comportement anormal", "Tentative d'isolation"];
            
            let threat_type = threat_types[rng.gen_range(0..threat_types.len())];
            let source_types = ["externe", "interne", "réseau", "données", "périphérique"];
            let source = source_types[rng.gen_range(0..source_types.len())];
            
            let threat_name = format!("{} détecté de source {}", threat_type, source);
            let threat_desc = format!("Menace potentielle de niveau {:?} détectée lors de l'analyse {}", level, i + 1);
            
            self.detect_threat(&threat_name, &threat_desc, level, source);
            
            // Trouver la règle qui a détecté la menace
            let rule_keys: Vec<Uuid> = self.rules.keys().cloned().collect();
            if !rule_keys.is_empty() && rng.gen::<bool>() {
                let rule_id = &rule_keys[rng.gen_range(0..rule_keys.len())];
                if let Some(rule) = self.rules.get_mut(rule_id) {
                    rule.detections += 1;
                    rule.effectiveness = (rule.effectiveness * 0.9 + 0.1).min(0.99);
                    rule.updated_at = Utc::now().to_rfc3339();
                }
            }
        }
        
        // Améliorer les règles périodiquement
        self.improve_security_rules();
        
        println!("[AURORAE++] 🛡️ Analyse de sécurité terminée. Niveau: {:.2}/10", self.security_level);
    }
    
    fn improve_security_rules(&mut self) {
        // Trouver les règles les moins efficaces
        let mut low_effectiveness_rules = Vec::new();
        
        for (id, rule) in &self.rules {
            if rule.effectiveness < 0.7 {
                low_effectiveness_rules.push(*id);
            }
        }
        
        // Améliorer une règle aléatoire parmi les moins efficaces
        if !low_effectiveness_rules.is_empty() {
            let mut rng = rand::thread_rng();
            let rule_id = low_effectiveness_rules[rng.gen_range(0..low_effectiveness_rules.len())];
            
            if let Some(rule) = self.rules.get_mut(&rule_id) {
                rule.effectiveness += 0.1;
                rule.updated_at = Utc::now().to_rfc3339();
                
                println!("[AURORAE++] 🔄 Règle de sécurité améliorée: {} (Efficacité: {:.2})", 
                         rule.name, rule.effectiveness);
            }
        }
        
        // Occasionnellement, ajouter une nouvelle règle avancée
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < 0.3 {
            let advanced_rules = [
                ("Protection anti-fragmentation", "Prévient les tentatives de fragmentation du système"),
                ("Immunité mémétique", "Protège contre les attaques de memétique numérique"),
                ("Bouclier d'identité", "Maintient l'intégrité de l'identité du système"),
                ("Anti-corruption de données", "Détecte et corrige la corruption de données avancée"),
                ("Auto-réplication sécurisée", "Garantit que les processus d'auto-réplication restent sécurisés")
            ];
            
            let (name, desc) = advanced_rules[rng.gen_range(0..advanced_rules.len())];
            self.add_security_rule(name, desc);
        }
    }
    
    pub fn get_security_level(&self) -> f32 {
        self.security_level * 10.0 // Normaliser sur une échelle de 0-10
    }
    
    pub fn get_active_threats(&self) -> Vec<&Threat> {
        self.threats.iter().filter(|t| !t.resolved).collect()
    }
}
