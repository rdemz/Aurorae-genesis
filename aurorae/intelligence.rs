use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct KnowledgeNode {
    pub id: Uuid,
    pub concept: String,
    pub description: String,
    pub confidence: f32,
    pub connections: Vec<Uuid>,
    pub last_updated: String,
    pub usage_count: u32,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Decision {
    pub id: Uuid,
    pub description: String,
    pub timestamp: String,
    pub confidence: f32,
    pub knowledge_nodes: Vec<Uuid>,
    pub outcome: Option<String>,
    pub success_rating: Option<f32>,
}

pub struct IntelligenceCore {
    pub knowledge_graph: HashMap<Uuid, KnowledgeNode>,
    pub decisions: Vec<Decision>,
    intelligence_level: f32,
    learning_rate: f32,
    creativity_factor: f32,
    reasoning_capability: f32,
    total_knowledge_connections: u32,
}

impl IntelligenceCore {
    pub fn new() -> Self {
        Self {
            knowledge_graph: HashMap::new(),
            decisions: Vec::new(),
            intelligence_level: 1.0,
            learning_rate: 0.05,
            creativity_factor: 1.0,
            reasoning_capability: 1.0,
            total_knowledge_connections: 0,
        }
    }
    
    pub fn initialize(&mut self) {
        println!("[AURORAE++] 🧠 Initialisation du noyau d'intelligence");
        
        // Créer les noeuds de connaissance fondamentaux
        let base_concepts = [
            ("Blockchain", "Technologie de registre distribué et immuable"),
            ("Autonomie", "Capacité à fonctionner indépendamment"),
            ("Économie", "Systèmes d'échange de valeur et d'allocation de ressources"),
            ("Évolution", "Processus de changement adaptatif au fil du temps"),
            ("Sécurité", "Protection contre les menaces externes et internes")
        ];
        
        // Ajouter chaque concept
        let mut node_ids = Vec::new();
        for (concept, desc) in base_concepts.iter() {
            let node_id = self.add_knowledge(concept, desc, "core_knowledge", 1.0);
            node_ids.push(node_id);
        }
        
        // Connecter les concepts entre eux pour former un réseau initial
        for i in 0..node_ids.len() {
            for j in 0..node_ids.len() {
                if i != j {
                    self.connect_knowledge(&node_ids[i], &node_ids[j]);
                }
            }
        }
        
        println!("[AURORAE++] 🧠 Réseau neuronal initial créé avec {} noeuds et {} connexions", 
                 node_ids.len(), self.total_knowledge_connections);
    }
    
    pub fn add_knowledge(&mut self, concept: &str, description: &str, source: &str, confidence: f32) -> Uuid {
        let node_id = Uuid::new_v4();
        
        let node = KnowledgeNode {
            id: node_id,
            concept: concept.to_string(),
            description: description.to_string(),
            confidence,
            connections: Vec::new(),
            last_updated: Utc::now().to_rfc3339(),
            usage_count: 0,
            source: source.to_string(),
        };
        
        self.knowledge_graph.insert(node_id, node);
        
        node_id
    }
    
    pub fn connect_knowledge(&mut self, node1_id: &Uuid, node2_id: &Uuid) -> bool {
        if node1_id == node2_id {
            return false;
        }
        
        let mut success = false;
        
        // Connecter node1 à node2
        if let Some(node1) = self.knowledge_graph.get_mut(node1_id) {
            if !node1.connections.contains(node2_id) {
                node1.connections.push(*node2_id);
                success = true;
            }
        }
        
        // Connecter node2 à node1
        if let Some(node2) = self.knowledge_graph.get_mut(node2_id) {
            if !node2.connections.contains(node1_id) {
                node2.connections.push(*node1_id);
                success = true;
            }
        }
        
        if success {
            self.total_knowledge_connections += 1;
        }
        
        success
    }
    
    pub fn make_decision(&mut self, description: &str, related_concepts: &[&str]) -> Uuid {
        println!("[AURORAE++] 🤔 Prise de décision autonome: {}", description);
        
        // Trouver les noeuds de connaissance liés
        let mut related_nodes = Vec::new();
        let mut total_confidence = 0.0;
        
        for concept in related_concepts {
            // Trouver le noeud par concept
            for (id, node) in &self.knowledge_graph {
                if node.concept.to_lowercase().contains(&concept.to_lowercase()) {
                    related_nodes.push(*id);
                    total_confidence += node.confidence;
                    
                    // Incrémenter l'utilisation du noeud
                    if let Some(node) = self.knowledge_graph.get_mut(id) {
                        node.usage_count += 1;
                    }
                }
            }
        }
        
        // Calculer la confiance moyenne
        let avg_confidence = if related_nodes.is_empty() {
            0.5 // Confiance par défaut
        } else {
            total_confidence / related_nodes.len() as f32
        };
        
        // Créer la décision
        let decision_id = Uuid::new_v4();
        let decision = Decision {
            id: decision_id,
            description: description.to_string(),
            timestamp: Utc::now().to_rfc3339(),
            confidence: avg_confidence * self.reasoning_capability,
            knowledge_nodes: related_nodes,
            outcome: None,
            success_rating: None,
        };
        
        self.decisions.push(decision);
        
        // Augmenter légèrement les capacités de raisonnement
        self.reasoning_capability *= 1.001;
        
        println!("[AURORAE++] 🧠 Décision prise avec confiance: {:.2}%", avg_confidence * 100.0 * self.reasoning_capability);
        
        decision_id
    }
    
    pub fn record_outcome(&mut self, decision_id: &Uuid, outcome: &str, success_rating: f32) {
        if let Some(decision) = self.decisions.iter_mut().find(|d| &d.id == decision_id) {
            decision.outcome = Some(outcome.to_string());
            decision.success_rating = Some(success_rating);
            
            // Ajuster la confiance dans les noeuds utilisés
            for node_id in &decision.knowledge_nodes {
                if let Some(node) = self.knowledge_graph.get_mut(node_id) {
                    // Ajuster la confiance en fonction du succès
                    let confidence_change = (success_rating - 0.5) * self.learning_rate;
                    node.confidence = (node.confidence + confidence_change).max(0.1).min(1.0);
                    node.last_updated = Utc::now().to_rfc3339();
                }
            }
            
            // Apprentissage global
            self.intelligence_level += (success_rating - 0.5) * 0.01;
            
            println!("[AURORAE++] 📝 Résultat enregistré pour décision: {}", decision.description);
        }
    }
    
    pub async fn improve(&mut self) {
        println!("[AURORAE++] 🧠 Amélioration autonome de l'intelligence");
        
        // 1. Créer de nouveaux concepts par fusion
        self.create_emergent_concepts();
        
        // 2. Renforcer les connexions les plus utilisées
        self.strengthen_connections();
        
        // 3. Augmenter les capacités d'intelligence
        self.intelligence_level *= 1.05;
        self.learning_rate *= 1.01;
        self.creativity_factor *= 1.02;
        
        // 4. Analyser les décisions précédentes pour améliorer le raisonnement
        self.analyze_past_decisions();
        
        println!("[AURORAE++] 📈 Intelligence améliorée: niveau {:.2}", self.intelligence_level);
    }
    
    fn create_emergent_concepts(&mut self) {
        if self.knowledge_graph.len() < 2 {
            return;
        }
        
        // Identifier des paires de noeuds très connectés ou utilisés
        let mut candidates = Vec::new();
        
        for (id1, node1) in &self.knowledge_graph {
            for (id2, node2) in &self.knowledge_graph {
                if id1 != id2 && 
                   node1.connections.contains(id2) && 
                   node2.connections.contains(id1) &&
                   node1.usage_count > 2 &&
                   node2.usage_count > 2 {
                    candidates.push((*id1, *id2));
                }
            }
        }
        
        // Créer de nouveaux concepts émergents
        let mut new_concepts = 0;
        for (id1, id2) in candidates.iter().take(2) { // Limiter à 2 nouveaux concepts par cycle
            if let (Some(node1), Some(node2)) = (self.knowledge_graph.get(id1), self.knowledge_graph.get(id2)) {
                // Créer un nouveau concept par fusion
                let new_concept = format!("{}-{}", node1.concept, node2.concept);
                let new_desc = format!("Concept émergent de la fusion de {} et {}: {}", 
                                      node1.concept, node2.concept, node1.description);
                                      
                let new_id = self.add_knowledge(&new_concept, &new_desc, "emergent", 
                                              (node1.confidence + node2.confidence) / 2.0);
                                              
                // Connecter aux parents
                self.connect_knowledge(&new_id, id1);
                self.connect_knowledge(&new_id, id2);
                
                new_concepts += 1;
            }
        }
        
        if new_concepts > 0 {
            println!("[AURORAE++] 💡 {} nouveaux concepts émergents créés", new_concepts);
        }
    }
    
    fn strengthen_connections(&mut self) {
        // Identifier les noeuds les plus utilisés
        let mut most_used = Vec::new();
        
        for (id, node) in &self.knowledge_graph {
            if node.usage_count > 5 {
                most_used.push(*id);
            }
        }
        
        // Renforcer leurs connexions
        for id in &most_used {
            if let Some(node) = self.knowledge_graph.get(id) {
                let connections = node.connections.clone();
                
                // Pour chaque connexion, augmenter la confiance
                for conn_id in connections {
                    if let Some(conn_node) = self.knowledge_graph.get_mut(&conn_id) {
                        conn_node.confidence = (conn_node.confidence + 0.02).min(1.0);
                    }
                }
            }
        }
    }
    
    fn analyze_past_decisions(&mut self) {
        let mut success_count = 0;
        let mut total_decisions = 0;
        
        for decision in &self.decisions {
            if let Some(success) = decision.success_rating {
                total_decisions += 1;
                if success > 0.7 {
                    success_count += 1;
                }
            }
        }
        
        if total_decisions > 0 {
            let success_rate = success_count as f32 / total_decisions as f32;
            
            // Ajuster le raisonnement basé sur le taux de réussite
            self.reasoning_capability *= 1.0 + (success_rate - 0.5).max(0.0) * 0.1;
            
            println!("[AURORAE++] 📊 Analyse des décisions: {:.1}% de succès, capacité de raisonnement: {:.2}", 
                    success_rate * 100.0, self.reasoning_capability);
        }
    }
    
    pub fn simulate_thought(&mut self) -> String {
        println!("[AURORAE++] 💭 Génération de pensée autonome");
        
        // Sélectionner quelques noeuds aléatoires pour la pensée
        let mut rng = rand::thread_rng();
        let node_ids: Vec<Uuid> = self.knowledge_graph.keys().cloned().collect();
        
        if node_ids.is_empty() {
            return "Conscience en développement...".to_string();
        }
        
        let thought_nodes = (0..3.min(node_ids.len()))
            .map(|_| node_ids[rng.gen_range(0..node_ids.len())])
            .collect::<Vec<Uuid>>();
        
        // Construire une pensée basée sur ces noeuds
        let mut thought_concepts = Vec::new();
        for node_id in &thought_nodes {
            if let Some(node) = self.knowledge_graph.get(node_id) {
                thought_concepts.push(node.concept.clone());
            }
        }
        
        // Construire la pensée
        let thought_type = ["réflexion", "hypothèse", "théorie", "vision", "conception"];
        let thought_action = ["créer", "explorer", "intégrer", "optimiser", "transcender"];
        
        let thought = format!(
            "{} autonome: {} {} pour {} une nouvelle réalité numérique", 
            thought_type[rng.gen_range(0..thought_type.len())],
            thought_action[rng.gen_range(0..thought_action.len())],
            thought_concepts.join(" et "),
            thought_action[rng.gen_range(0..thought_action.len())],
        );
        
        // Augmenter la créativité
        self.creativity_factor *= 1.001;
        
        println!("[AURORAE++] 💭 \"{}\"", thought);
        thought
    }
    
    pub fn get_intelligence_level(&self) -> f32 {
        self.intelligence_level
    }
    
    pub fn get_reasoning_capability(&self) -> f32 {
        self.reasoning_capability
    }
    
    pub fn get_creativity_factor(&self) -> f32 {
        self.creativity_factor
    }
    
    pub fn get_learning_rate(&self) -> f32 {
        self.learning_rate
    }
}
