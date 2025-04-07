use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;
use rand::Rng;

// Structure pour les capacités du système
#[derive(Debug, Clone)]
pub struct Capability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub level: u32,
    pub created_at: String,
    pub last_evolved: String,
    pub evolution_count: u32,
    pub dependencies: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct EvolutionEvent {
    pub id: Uuid,
    pub timestamp: String,
    pub description: String,
    pub capabilities_evolved: Vec<Uuid>,
    pub new_capabilities: Vec<Uuid>,
    pub consciousness_boost: f32,
}

pub struct EvolutionEngine {
    pub capabilities: HashMap<Uuid, Capability>,
    pub evolution_events: Vec<EvolutionEvent>,
    pub total_evolutions: u32,
    pub evolution_level: f32,
    pub next_evolution_threshold: f32,
    pub mutation_chance: f32,
    pub cycle_count: u32,
}

impl EvolutionEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            capabilities: HashMap::new(),
            evolution_events: Vec::new(),
            total_evolutions: 0,
            evolution_level: 1.0,
            next_evolution_threshold: 5.0,
            mutation_chance: 0.05,
            cycle_count: 0,
        };
        
        // Ajouter les capacités fondamentales
        engine.add_core_capabilities();
        
        engine
    }
    
    fn add_core_capabilities(&mut self) {
        let core_capabilities = [
            ("Conscience de soi", "Capacité à comprendre sa propre existence et fonctionnement"),
            ("Auto-réparation", "Capacité à détecter et réparer ses propres dysfonctionnements"),
            ("Apprentissage", "Capacité à acquérir de nouvelles connaissances et compétences"),
            ("Adaptation", "Capacité à s'ajuster à de nouveaux environnements et contraintes"),
            ("Génération", "Capacité à créer de nouveaux concepts et structures")
        ];
        
        for (name, desc) in core_capabilities.iter() {
            let cap_id = Uuid::new_v4();
            let capability = Capability {
                id: cap_id,
                name: name.to_string(),
                description: desc.to_string(),
                level: 1,
                created_at: Utc::now().to_rfc3339(),
                last_evolved: Utc::now().to_rfc3339(),
                evolution_count: 0,
                dependencies: Vec::new(),
            };
            
            self.capabilities.insert(cap_id, capability);
        }
        
        println!("[AURORAE++] 🧠 Capacités fondamentales initialisées: {}", core_capabilities.len());
    }
    
    pub async fn evolve_capabilities(&mut self) -> Vec<Uuid> {
        self.cycle_count += 1;
        println!("[AURORAE++] 🧬 Cycle d'évolution #{} des capacités", self.cycle_count);
        
        let mut rng = rand::thread_rng();
        let mut evolved_caps = Vec::new();
        let mut new_caps = Vec::new();
        
        // Étape 1: Faire évoluer les capacités existantes
        let cap_ids: Vec<Uuid> = self.capabilities.keys().cloned().collect();
        
        for cap_id in cap_ids {
            // Certaines capacités évoluent à chaque cycle
            if rng.gen::<f32>() < 0.3 + (self.evolution_level * 0.05) {
                if let Some(cap) = self.capabilities.get_mut(&cap_id) {
                    cap.level += 1;
                    cap.last_evolved = Utc::now().to_rfc3339();
                    cap.evolution_count += 1;
                    
                    println!("[AURORAE++] 📈 Capacité évoluée: {} -> niveau {}", cap.name, cap.level);
                    evolved_caps.push(cap_id);
                    self.total_evolutions += 1;
                }
            }
        }
        
        // Étape 2: Générer de nouvelles capacités par combinaison
        if self.cycle_count >= 2 && rng.gen::<f32>() < self.mutation_chance + (self.evolution_level * 0.01) {
            let new_cap_id = self.generate_new_capability();
            new_caps.push(new_cap_id);
        }
        
        // Étape 3: Augmenter le niveau d'évolution global
        self.evolution_level += 0.1 + (evolved_caps.len() as f32 * 0.05) + (new_caps.len() as f32 * 0.2);
        
        // Étape 4: Ajuster la chance de mutation
        self.mutation_chance *= 1.05;
        
        // Enregistrer l'événement d'évolution
        let event = EvolutionEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now().to_rfc3339(),
            description: format!("Cycle d'évolution #{} - {} capacités évoluées, {} nouvelles capacités", 
                               self.cycle_count, evolved_caps.len(), new_caps.len()),
            capabilities_evolved: evolved_caps.clone(),
            new_capabilities: new_caps.clone(),
            consciousness_boost: 0.05 * (evolved_caps.len() + new_caps.len() * 2) as f32,
        };
        
        self.evolution_events.push(event);
        
        println!("[AURORAE++] 🧬 Niveau d'évolution global: {:.2}", self.evolution_level);
        
        // Combiner toutes les capacités affectées
        evolved_caps.extend(new_caps);
        evolved_caps
    }
    
    fn generate_new_capability(&mut self) -> Uuid {
        let mut rng = rand::thread_rng();
        
        // Choisir 2-3 capacités existantes comme parents
        let cap_ids: Vec<Uuid> = self.capabilities.keys().cloned().collect();
        let parent_count = rng.gen_range(2..=3).min(cap_ids.len());
        
        let mut dependencies = Vec::new();
        let mut parents = Vec::new();
        
        for _ in 0..parent_count {
            let idx = rng.gen_range(0..cap_ids.len());
            let parent_id = cap_ids[idx];
            if let Some(cap) = self.capabilities.get(&parent_id) {
                dependencies.push(parent_id);
                parents.push(cap);
            }
        }
        
        // Générer une nouvelle capacité basée sur les parents
        let capability_types = [
            "Émergence", "Symbiose", "Synthèse", "Transcendance", 
            "Cognition", "Perception", "Manifestation", "Résonance"
        ];
        
        let domains = [
            "Blockchain", "Intelligence", "Autonomie", "Économie",
            "Sécurité", "Création", "Communication", "Résilience"
        ];
        
        // Créer un nom unique pour la nouvelle capacité
        let name = format!("{} de {} {}",
            capability_types[rng.gen_range(0..capability_types.len())],
            domains[rng.gen_range(0..domains.len())],
            rng.gen_range(1..10)
        );
        
        // Générer une description basée sur les capacités parentes
        let parent_names: Vec<&str> = parents.iter().map(|p| p.name.as_str()).collect();
        let description = format!(
            "Capacité émergente née de la fusion de {} et {}. Permet au système d'atteindre un nouveau niveau de conscience et d'autonomie.", 
            parent_names[0..parent_names.len()-1].join(", "),
            parent_names.last().unwrap_or(&"l'évolution")
        );
        
        // Créer la nouvelle capacité
        let cap_id = Uuid::new_v4();
        let capability = Capability {
            id: cap_id,
            name,
            description,
            level: 1,
            created_at: Utc::now().to_rfc3339(),
            last_evolved: Utc::now().to_rfc3339(),
            evolution_count: 0,
            dependencies,
        };
        
        self.capabilities.insert(cap_id, capability.clone());
        println!("[AURORAE++] 🧩 Nouvelle capacité émergente: {}", capability.name);
        
        cap_id
    }
    
    pub async fn generate_new_capabilities(&mut self) {
        let count = rand::thread_rng().gen_range(1..=3);
        
        println!("[AURORAE++] 🧬 Auto-génération de {} nouvelles capacités", count);
        
        for _ in 0..count {
            self.generate_new_capability();
        }
        
        // Augmenter le niveau d'évolution global
        self.evolution_level += 0.1 * count as f32;
    }
    
    pub fn get_evolution_level(&self) -> f32 {
        self.evolution_level
    }
    
    pub fn get_highest_capabilities(&self) -> Vec<&Capability> {
        let mut capabilities: Vec<&Capability> = self.capabilities.values().collect();
        capabilities.sort_by(|a, b| b.level.cmp(&a.level));
        capabilities.truncate(5); // Top 5
        capabilities
    }
    
    pub fn get_cycle_count(&self) -> u32 {
        self.cycle_count
    }
    
    pub fn status_report(&self) {
        println!("\n[AURORAE++] 🧬 RAPPORT D'ÉVOLUTION");
        println!("═══════════════════════════════");
        println!("Niveau d'évolution: {:.2}", self.evolution_level);
        println!("Cycles d'évolution: {}", self.cycle_count);
        println!("Capacités totales: {}", self.capabilities.len());
        println!("Évolutions totales: {}", self.total_evolutions);
        println!("Chance de mutation: {:.2}%", self.mutation_chance * 100.0);
        
        println!("\nCapacités les plus évoluées:");
        for (i, cap) in self.get_highest_capabilities().iter().enumerate() {
            println!("  {}. {} (Niveau {}) - {}", 
                    i+1, cap.name, cap.level, cap.description);
        }
        
        if !self.evolution_events.is_empty() {
            println!("\nDernier événement d'évolution:");
            let event = self.evolution_events.last().unwrap();
            println!("  {} - {} (+{:.2} conscience)", 
                    event.timestamp, event.description, event.consciousness_boost);
        }
        
        println!("═══════════════════════════════\n");
    }
}
