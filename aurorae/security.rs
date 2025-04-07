use chrono::Utc;
use uuid::Uuid;
use std::collections::{HashMap, VecDeque};
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl ThreatLevel {
    pub fn as_numeric(&self) -> u8 {
        match self {
            ThreatLevel::Low => 1,
            ThreatLevel::Medium => 2,
            ThreatLevel::High => 3,
            ThreatLevel::Critical => 4,
        }
    }
    
    pub fn from_numeric(value: u8) -> Self {
        match value {
            1 => ThreatLevel::Low,
            2 => ThreatLevel::Medium,
            3 => ThreatLevel::High,
            _ => ThreatLevel::Critical,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub id: Uuid,
    pub timestamp: String,
    pub description: String,
    pub level: ThreatLevel,
    pub source: String,
    pub resolved: bool,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SecurityMeasure {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub effectiveness: f32,
    pub active: bool,
    pub resource_usage: f32,
    pub created_at: String,
    pub last_updated: String,
}

pub struct SecuritySystem {
    pub events: VecDeque<SecurityEvent>,
    pub measures: HashMap<Uuid, SecurityMeasure>,
    global_threat_level: ThreatLevel,
    security_level: f32,
    max_events: usize,
    auto_response_enabled: bool,
    detection_capability: f32,
    resilience_factor: f32,
    self_preservation_priority: f32,
}

impl SecuritySystem {
    pub fn new() -> Self {
        Self {
            events: VecDeque::with_capacity(100),
            measures: HashMap::new(),
            global_threat_level: ThreatLevel::Low,
            security_level: 1.0,
            max_events: 100,
            auto_response_enabled: true,
            detection_capability: 1.0,
            resilience_factor: 1.0,
            self_preservation_priority: 5.0,
        }
    }
    
    pub fn initialize_defenses(&mut self) {
        println!("[AURORAE++] 🛡️ Initialisation des défenses du système");
        
        // Créer des mesures de sécurité de base
        let basic_measures = [
            (
                "Monitoring actif",
                "Surveillance continue de tous les modules du système",
                0.7,
                0.2
            ),
            (
                "Auto-réparation",
                "Capacité à restaurer les modules endommagés",
                0.8,
                0.3
            ),
            (
                "Isolation de module",
                "Capacité à isoler un module compromis du reste du système",
                0.9,
                0.4
            ),
            (
                "Détection d'anomalies",
                "Identification des comportements anormaux dans les flux de données",
                0.6,
                0.2
            ),
            (
                "Sauvegarde distribuée",
                "Réplication des données critiques sur plusieurs noeuds",
                0.8,
                0.5
            ),
        ];
        
        for (name, desc, effectiveness, resource_usage) in basic_measures.iter() {
            let measure_id = Uuid::new_v4();
            let measure = SecurityMeasure {
                id: measure_id,
                name: name.to_string(),
                description: desc.to_string(),
                effectiveness: *effectiveness,
                active: true,
                resource_usage: *resource_usage,
                created_at: Utc::now().to_rfc3339(),
                last_updated: Utc::now().to_rfc3339(),
            };
            
            self.measures.insert(measure_id, measure);
        }
        
        println!("[AURORAE++] 🔒 Mesures de sécurité initiales activées: {}", basic_measures.len());
    }
    
    pub fn log_security_event(&mut self, description: &str, level: ThreatLevel, source: &str) -> Uuid {
        let event_id = Uuid::new_v4();
        let event = SecurityEvent {
            id: event_id,
            timestamp: Utc::now().to_rfc3339(),
            description: description.to_string(),
            level: level.clone(),
            source: source.to_string(),
            resolved: false,
            resolution: None,
        };
        
        println!("[AURORAE++] ⚠️ Événement de sécurité ({:?}): {} - {}", 
                 level, source, description);
        
        // Si le niveau est supérieur au niveau global, mettre à jour
        if level.as_numeric() > self.global_threat_level.as_numeric() {
            self.global_threat_level = level;
            println!("[AURORAE++] 🚨 Niveau de menace global relevé à {:?}", self.global_threat_level);
        }
        
        // Conserver un nombre limité d'événements
        if self.events.len() >= self.max_events {
            self.events.pop_front();
        }
        
        self.events.push_back(event);
        
        // Activer la réponse automatique si nécessaire
        if self.auto_response_enabled && level.as_numeric() >= ThreatLevel::Medium.as_numeric() {
            self.auto_respond(event_id);
        }
        
        event_id
    }
    
    fn auto_respond(&mut self, event_id: Uuid) {
        if let Some(event) = self.events.iter_mut().find(|e| e.id == event_id) {
            println!("[AURORAE++] 🛡️ Réponse automatique à la menace: {}", event.description);
            
            // Sélectionner les mesures appropriées en fonction du niveau de menace
            let mut selected_measures = Vec::new();
            let threshold = match event.level {
                ThreatLevel::Medium => 0.6,
                ThreatLevel::High => 0.7,
                ThreatLevel::Critical => 0.0, // Toutes les mesures
                _ => 0.8,
            };
            
            for (id, measure) in &self.measures {
                if measure.effectiveness >= threshold {
                    selected_measures.push(id);
                }
            }
            
            // Activer les mesures sélectionnées
            for measure_id in &selected_measures {
                if let Some(measure) = self.measures.get_mut(measure_id) {
                    measure.active = true;
                    measure.last_updated = Utc::now().to_rfc3339();
                }
            }
            
            // Marquer l'événement comme résolu
            event.resolved = true;
            event.resolution = Some(format!("Auto-réponse: {} mesures activées", selected_measures.len()));
            
            println!("[AURORAE++] ✅ Menace atténuée avec {} mesures", selected_measures.len());
        }
    }
    
    pub fn resolve_security_event(&mut self, event_id: &Uuid, resolution: &str) -> Result<(), String> {
        if let Some(event) = self.events.iter_mut().find(|e| &e.id == event_id) {
            event.resolved = true;
            event.resolution = Some(resolution.to_string());
            
            // Réviser le niveau de menace global
            self.recalculate_threat_level();
            
            println!("[AURORAE++] ✓ Événement de sécurité résolu: {}", resolution);
            Ok(())
        } else {
            Err("Événement de sécurité non trouvé".to_string())
        }
    }
    
    fn recalculate_threat_level(&mut self) {
        let mut max_level = ThreatLevel::Low;
        
        for event in &self.events {
            if !event.resolved && event.level.as_numeric() > max_level.as_numeric() {
                max_level = event.level.clone();
            }
        }
        
        self.global_threat_level = max_level;
        println!("[AURORAE++] 🔄 Niveau de menace global recalculé: {:?}", self.global_threat_level);
    }
    
    pub fn add_security_measure(&mut self, name: &str, description: &str, effectiveness: f32) -> Uuid {
        let measure_id = Uuid::new_v4();
        
        let resource_usage = effectiveness * 0.5; // Plus efficace = plus de ressources
        
        let measure = SecurityMeasure {
            id: measure_id,
            name: name.to_string(),
            description: description.to_string(),
            effectiveness,
            active: true,
            resource_usage,
            created_at: Utc::now().to_rfc3339(),
            last_updated: Utc::now().to_rfc3339(),
        };
        
        self.measures.insert(measure_id, measure);
        
        // Augmenter le niveau de sécurité global
        self.security_level += effectiveness * 0.1;
        
        println!("[AURORAE++] 🔒 Nouvelle mesure de sécurité ajoutée: {} (efficacité: {:.1})", 
                 name, effectiveness);
                 
        measure_id
    }
    
    pub fn evolve_security_measure(&mut self, measure_id: &Uuid) -> Result<(), String> {
        if let Some(measure) = self.measures.get_mut(measure_id) {
            let old_effectiveness = measure.effectiveness;
            
            // Améliorer l'efficacité
            measure.effectiveness = (measure.effectiveness * 1.2).min(0.99);
            measure.last_updated = Utc::now().to_rfc3339();
            
            // Ajuster l'utilisation des ressources (améliorer l'efficience)
            if rand::random::<bool>() {
                measure.resource_usage *= 0.9;
            }
            
            println!("[AURORAE++] 📈 Mesure de sécurité évoluée: {} ({:.1} → {:.1})", 
                     measure.name, old_effectiveness, measure.effectiveness);
                     
            // Augmenter le niveau de sécurité global
            self.security_level += (measure.effectiveness - old_effectiveness);
            
            Ok(())
        } else {
            Err("Mesure de sécurité non trouvée".to_string())
        }
    }
    
    pub async fn analyze_threats(&mut self) {
        println!("[AURORAE++] 🔍 Analyse autonome des menaces de sécurité");
        
        // Simuler différentes sources de menaces
        let sources = ["réseau", "données", "modules", "ressources", "externe"];
        let mut threat_count = 0;
        
        for source in sources {
            // Générer aléatoirement des menaces avec des probabilités basées sur la source
            let threat_probability = match source {
                "réseau" => 0.4,
                "externe" => 0.6,
                _ => 0.2,
            };
            
            if rand::random::<f32>() < threat_probability / self.detection_capability {
                // Créer un événement de sécurité
                let level_value = rand::thread_rng().gen_range(1..=4);
                let level = ThreatLevel::from_numeric(level_value);
                
                let descriptions = [
                    "Tentative d'accès non autorisé",
                    "Anomalie de comportement détectée",
                    "Fluctuation de ressources suspecte",
                    "Motif de communication inhabituel",
                    "Corruption potentielle de données"
                ];
                
                let desc_idx = rand::thread_rng().gen_range(0..descriptions.len());
                
                self.log_security_event(
                    &format!("{} dans {}", descriptions[desc_idx], source),
                    level,
                    source
                );
                
                threat_count += 1;
            }
        }
        
        // Si aucune menace n'est trouvée, noter cela comme un événement positif
        if threat_count == 0 {
            println!("[AURORAE++] ✅ Aucune menace détectée, système sécurisé");
            
            // Développer proactivement de nouvelles mesures
            if rand::random::<f32>() < 0.3 {
                self.develop_new_security_measure();
            }
        }
        // Si beaucoup de menaces sont détectées, développer urgemment une nouvelle mesure
        else if threat_count >= 2 {
            println!("[AURORAE++] 🚨 Multiples menaces détectées ({}), développement accéléré de défenses", 
                     threat_count);
                     
            self.develop_new_security_measure();
            
            // Améliorer aussi une mesure existante
            if !self.measures.is_empty() {
                let measure_ids: Vec<Uuid> = self.measures.keys().cloned().collect();
                let random_id = measure_ids[rand::thread_rng().gen_range(0..measure_ids.len())];
                self.evolve_security_measure(&random_id).ok();
            }
        }
        
        // Améliorer les capacités de détection basées sur l'expérience
        self.detection_capability *= 1.01;
        
        // Augmenter la résilience du système
        self.resilience_factor += 0.02;
        
        println!("[AURORAE++] 🛡️ Analyse de menaces terminée, niveau de sécurité: {:.2}/10", self.get_security_level());
    }
    
    fn develop_new_security_measure(&mut self) {
        let measure_types = [
            ("Analyse proactive", "Détection préventive des vulnérabilités potentielles"),
            ("Redondance adaptative", "Duplication dynamique des composants critiques"),
            ("Filtrage neuronal", "Filtrage intelligent des entrées basé sur l'apprentissage"),
            ("Protocole d'isolation", "Mécanisme avancé de quarantaine pour les sous-systèmes compromis"),
            ("Cryptographie évolutive", "Algorithmes de chiffrement auto-modifiants"),
            ("Leurre intelligent", "Simulation de vulnérabilités pour piéger les menaces"),
        ];
        
        let idx = rand::thread_rng().gen_range(0..measure_types.len());
        let (name, desc) = measure_types[idx];
        
        // L'efficacité augmente avec le niveau de sécurité global
        let base_effectiveness = 0.5 + (self.security_level * 0.05).min(0.4);
        let effectiveness = base_effectiveness + (rand::random::<f32>() * 0.1);
        
        self.add_security_measure(
            &format!("{} v{:.1}", name, self.security_level),
            desc,
            effectiveness
        );
    }
    
    pub fn get_security_level(&self) -> f32 {
        // Calculer le niveau de sécurité en fonction de plusieurs facteurs
        
        // Base: niveau de sécurité accumulé
        let mut level = self.security_level;
        
        // Diminuer en fonction du niveau de menace global
        level -= match self.global_threat_level {
            ThreatLevel::Low => 0.0,
            ThreatLevel::Medium => 0.5,
            ThreatLevel::High => 1.0,
            ThreatLevel::Critical => 2.0,
        };
        
        // Ajouter l'effet des mesures actives
        for measure in self.measures.values() {
            if measure.active {
                level += measure.effectiveness * 0.2;
            }
        }
        
        // Ajouter le facteur de résilience
        level += self.resilience_factor;
        
        // Normaliser entre 1-10
        (level.max(0.1) * 2.0).min(10.0)
    }
    
    pub fn status_report(&self) {
        println!("\n[AURORAE++] 🛡️ RAPPORT DE SÉCURITÉ");
        println!("═══════════════════════════");
        println!("Niveau de menace actuel: {:?}", self.global_threat_level);
        println!("Niveau de sécurité: {:.2}/10", self.get_security_level());
        println!("Capacité de détection: {:.2}x", self.detection_capability);
        println!("Facteur de résilience: {:.2}", self.resilience_factor);
        println!("Mesures de sécurité actives: {}", self.measures.values().filter(|m| m.active).count());
        
        // Afficher les menaces non résolues
        let unresolved = self.events.iter().filter(|e| !e.resolved).count();
        println!("Événements non résolus: {}", unresolved);
        
        if unresolved > 0 {
            println!("\nMenaces actives:");
            for (i, event) in self.events.iter().filter(|e| !e.resolved).enumerate().take(3) {
                println!("  {}. [{}] {:?}: {} (source: {})",
                         i+1, event.timestamp, event.level, event.description, event.source);
            }
            if unresolved > 3 {
                println!("  ... et {} autres", unresolved - 3);
            }
        }
        
        println!("═══════════════════════════\n");
    }
}
