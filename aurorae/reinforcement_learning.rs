use std::collections::{HashMap, HashSet};
use rand::Rng;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Définition de la structure de mémoire à long terme, permettant à l'agent de "rêver"
#[derive(Clone, Serialize, Deserialize)]
pub struct EpisodeMemory {
    pub state_history: Vec<String>,
    pub action_history: Vec<String>,
    pub reward_history: Vec<f32>,
    pub total_reward: f32,
    pub timestamp: u64,
    pub performance_score: f32,
}

// Structure représentant une stratégie que l'agent peut développer
#[derive(Clone, Serialize, Deserialize)]
pub struct Strategy {
    pub name: String,
    pub state_action_map: HashMap<String, String>,
    pub effectiveness: f32,
    pub usage_count: u32,
    pub last_updated: u64,
    pub creation_context: String,
}

// Structure principale de l'agent avec capacités étendues
#[derive(Clone, Serialize, Deserialize)]
pub struct LearningAgent {
    pub actions: Vec<String>,                         // Actions disponibles
    pub state: String,                                // État actuel
    pub q_table: HashMap<String, HashMap<String, f32>>, // Table Q traditionnelle
    pub learning_rate: f32,                           // Taux d'apprentissage
    pub discount_factor: f32,                         // Facteur de réduction
    pub exploration_rate: f32,                        // Taux d'exploration
    
    // Nouvelles fonctionnalités pour l'autonomie
    pub long_term_memory: Vec<EpisodeMemory>,         // Mémoire à long terme (pour "rêver")
    pub strategies: Vec<Strategy>,                    // Stratégies développées
    pub performance_history: Vec<(u64, f32)>,         // Historique des performances
    pub evolution_count: u32,                         // Nombre d'évolutions
    pub known_states: HashSet<String>,                // États découverts
    pub creation_timestamp: u64,                      // Moment de création
    pub last_evolution_timestamp: u64,                // Dernière évolution
    pub adaptation_threshold: f32,                    // Seuil pour s'adapter
    pub evolution_threshold: f32,                     // Seuil pour évoluer
    pub meta_learning_rate: f32,                      // Taux d'apprentissage sur les hyperparamètres
    pub current_episode: EpisodeMemory,               // Épisode en cours
    pub network_complexity: u32,                      // Complexité du réseau
}

impl LearningAgent {
    pub fn new(actions: Vec<String>, initial_state: &str) -> Self {
        let mut q_table = HashMap::new();
        for action in &actions {
            let mut action_map = HashMap::new();
            action_map.insert(initial_state.to_string(), 0.0);
            q_table.insert(action.clone(), action_map);
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();

        let mut known_states = HashSet::new();
        known_states.insert(initial_state.to_string());

        LearningAgent {
            actions: actions.clone(),
            state: initial_state.to_string(),
            q_table,
            learning_rate: 0.1,
            discount_factor: 0.9,
            exploration_rate: 0.1,
            
            // Initialisation des nouvelles propriétés
            long_term_memory: Vec::new(),
            strategies: Vec::new(),
            performance_history: vec![(current_time, 0.0)],
            evolution_count: 0,
            known_states,
            creation_timestamp: current_time,
            last_evolution_timestamp: current_time,
            adaptation_threshold: 0.2,
            evolution_threshold: 0.5,
            meta_learning_rate: 0.01,
            current_episode: EpisodeMemory {
                state_history: vec![initial_state.to_string()],
                action_history: Vec::new(),
                reward_history: Vec::new(),
                total_reward: 0.0,
                timestamp: current_time,
                performance_score: 0.0,
            },
            network_complexity: 1,
        }
    }

    // Fonction originale améliorée pour choisir une action
    pub fn choose_action(&mut self) -> String {
        let mut rng = rand::thread_rng();

        // Nouvelle logique: parfois utiliser une stratégie si une est disponible et efficace
        if !self.strategies.is_empty() && rng.gen::<f32>() < 0.2 {
            let strategy_index = rng.gen_range(0..self.strategies.len());
            if let Some(action) = self.strategies[strategy_index].state_action_map.get(&self.state) {
    // Emprunt immuable pour récupérer l'action
    let action_to_return = action.clone();
    
    // Emprunt mutable pour mettre à jour usage_count
    self.strategies[strategy_index].usage_count += 1;

    // Retourner l'action clonée
    return action_to_return;
                // Utilisation d'une stratégie connue pour cet état
                self.strategies[strategy_index].usage_count += 1;
                return action.clone();
            }
        }

        // Exploration vs exploitation (logique originale améliorée)
        if rng.gen::<f32>() < self.exploration_rate {
            let action = &self.actions[rng.gen_range(0..self.actions.len())];
            action.to_string()
        } else {
            // Exploitation mais avec biais pour favoriser les actions moins utilisées parmi les meilleures
            let best_actions = self.find_top_actions(3); // Top 3 actions
            
            if !best_actions.is_empty() {
                let chosen_index = rng.gen_range(0..best_actions.len());
                best_actions[chosen_index].clone()
            } else {
                // Fallback au cas où
                let action = &self.actions[rng.gen_range(0..self.actions.len())];
                action.to_string()
            }
        }
    }

    // Méthode pour trouver les meilleures actions pour un état
    fn find_top_actions(&self, n: usize) -> Vec<String> {
        let mut action_values: Vec<(String, f32)> = self.actions.iter()
            .filter_map(|action| {
                match self.q_table.get(action) {
                    Some(action_map) => {
                        match action_map.get(&self.state) {
                            Some(value) => Some((action.clone(), *value)),
                            None => Some((action.clone(), 0.0)),
                        }
                    },
                    None => None,
                }
            })
            .collect();
        
        // Tri des actions par valeur Q
        action_values.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Prendre les N meilleures actions
        action_values.iter()
            .take(n)
            .map(|(action, _)| action.clone())
            .collect()
    }

    // Fonction update_q_value avec la correction du borrowing
    pub fn update_q_value(&mut self, action: &str, reward: f32, next_state: &str) {
        // Ajouter l'état à notre liste d'états connus s'il est nouveau
        if !self.known_states.contains(next_state) {
            self.known_states.insert(next_state.to_string());
            
            // Initialiser les entrées de Q-table pour ce nouvel état
            for action in &self.actions {
                self.q_table.entry(action.clone())
                    .or_insert_with(HashMap::new)
                    .entry(next_state.to_string())
                    .or_insert(0.0);
            }
        }
        
        // Calculer d'abord la valeur Q maximale pour le prochain état
        let future_q_values: Vec<f32> = self.actions.iter()
            .filter_map(|a| {
                if let Some(action_map) = self.q_table.get(a) {
                    action_map.get(next_state).cloned()
                } else {
                    None
                }
            })
            .collect();
        
        // Trouver la valeur maximale
        let max_future_q = if !future_q_values.is_empty() {
            *future_q_values.iter().max_by(|a, b| 
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(&0.0)
        } else {
            0.0
        };

        // Mettre à jour la valeur Q actuelle
        let current_q_value = self.q_table
            .entry(action.to_string())
            .or_insert_with(HashMap::new)
            .entry(self.state.clone())
            .or_insert(0.0);

        // Calculer la nouvelle Q-value avec un facteur d'influence du réseau de complexité
        let complexity_factor = (1.0 + self.network_complexity as f32 / 10.0).min(2.0);
        let new_q_value = *current_q_value + self.learning_rate * complexity_factor * 
            (reward + self.discount_factor * max_future_q - *current_q_value);
        *current_q_value = new_q_value;
    }

    // Fonction d'apprentissage améliorée qui enregistre l'historique
    pub fn learn(&mut self, reward: f32, next_state: &str) {
        let action = self.choose_action();
        
        // Mettre à jour la mémoire de l'épisode en cours
        self.current_episode.state_history.push(self.state.clone());
        self.current_episode.action_history.push(action.clone());
        self.current_episode.reward_history.push(reward);
        self.current_episode.total_reward += reward;
        
        // Mettre à jour la Q-table
        self.update_q_value(&action, reward, next_state);
        
        // Mettre à jour l'état courant
        self.state = next_state.to_string();
        
        // Vérifier s'il faut s'adapter ou évoluer
        self.check_for_adaptation();
    }

    // Nouvelle fonction : Évaluer les performances actuelles
    pub fn evaluate_performance(&mut self) -> f32 {
        // Calculer la performance basée sur diverses métriques
        
        // 1. Score basé sur les récompenses récentes
        let recent_rewards_score = if self.current_episode.reward_history.len() > 10 {
            let recent_rewards: Vec<&f32> = self.current_episode.reward_history.iter()
                .rev().take(10).collect();
            recent_rewards.iter().map(|&r| *r).sum::<f32>() / recent_rewards.len() as f32
        } else if !self.current_episode.reward_history.is_empty() {
            self.current_episode.reward_history.iter().sum::<f32>() / 
                self.current_episode.reward_history.len() as f32
        } else {
            0.0
        };
        
        // 2. Score basé sur la diversité des états visités
        let exploration_ratio = self.known_states.len() as f32 / 
            (10.0 + self.evolution_count as f32 * 5.0);
        let exploration_score = (1.0 - (-exploration_ratio).exp()).min(1.0);
        
        // 3. Score basé sur l'efficacité des stratégies
        let strategy_score = if !self.strategies.is_empty() {
            self.strategies.iter()
                .map(|s| s.effectiveness)
                .sum::<f32>() / self.strategies.len() as f32
        } else {
            0.0
        };
        
        // Combiner les scores avec des poids
        let performance = recent_rewards_score * 0.6 + 
                          exploration_score * 0.3 + 
                          strategy_score * 0.1;
        
        // Enregistrer la performance
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        self.performance_history.push((current_time, performance));
        self.current_episode.performance_score = performance;
        
        // Si l'épisode est suffisamment long, l'archiver dans la mémoire à long terme
        if self.current_episode.state_history.len() > 50 {
            self.archive_current_episode();
        }
        
        performance
    }

    // Nouvelle fonction : Archiver l'épisode actuel dans la mémoire à long terme
    fn archive_current_episode(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        // Créer une copie de l'épisode actuel
        let episode_to_archive = self.current_episode.clone();
        
        // Ajouter à la mémoire à long terme
        self.long_term_memory.push(episode_to_archive);
        
        // Limiter la taille de la mémoire (garder les 100 meilleurs épisodes)
        if self.long_term_memory.len() > 100 {
            // Trier par performance et ne garder que les 100 meilleurs
            self.long_term_memory.sort_by(|a, b| 
                b.performance_score.partial_cmp(&a.performance_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            );
            self.long_term_memory.truncate(100);
        }
        
        // Réinitialiser l'épisode courant
        self.current_episode = EpisodeMemory {
            state_history: vec![self.state.clone()],
            action_history: Vec::new(),
            reward_history: Vec::new(),
            total_reward: 0.0,
            timestamp: current_time,
            performance_score: 0.0,
        };
    }

    // Nouvelle fonction : Vérifier s'il faut s'adapter ou évoluer
    fn check_for_adaptation(&mut self) {
        // Évaluer les performances actuelles
        let performance = self.evaluate_performance();
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        // Vérifier s'il est temps de s'adapter (toutes les 100 étapes)
        if self.current_episode.state_history.len() % 100 == 0 {
            // Si performance en dessous du seuil d'adaptation
            if performance < self.adaptation_threshold {
                self.adapt_parameters();
            }
            
            // Si performance en dessous du seuil d'évolution et assez de temps depuis la dernière évolution
            if performance < self.evolution_threshold && 
               current_time - self.last_evolution_timestamp > 3600 { // 1 heure
                self.evolve_network();
            }
            
            // Générer de nouvelles stratégies périodiquement
            if self.current_episode.state_history.len() % 500 == 0 {
                self.generate_strategy();
            }
            
            // "Rêver" périodiquement pour consolider l'apprentissage
            if self.current_episode.state_history.len() % 1000 == 0 && !self.long_term_memory.is_empty() {
                self.dream();
            }
        }
    }

    // Nouvelle fonction : Adapter les hyperparamètres en fonction des performances
    fn adapt_parameters(&mut self) {
        let mut rng = rand::thread_rng();
        
        // Ajuster le taux d'exploration
        if rng.gen::<f32>() < 0.5 {
            // Augmenter l'exploration si la performance est faible
            self.exploration_rate = (self.exploration_rate + self.meta_learning_rate).min(0.5);
        } else {
            // Diminuer l'exploration si la performance est bonne
            self.exploration_rate = (self.exploration_rate - self.meta_learning_rate).max(0.01);
        }
        
        // Ajuster le taux d'apprentissage
        if rng.gen::<f32>() < 0.3 {
            // Augmenter le taux d'apprentissage occasionnellement
            self.learning_rate = (self.learning_rate + self.meta_learning_rate * 0.5).min(0.3);
        } else {
            // Diminuer progressivement le taux d'apprentissage
            self.learning_rate = (self.learning_rate - self.meta_learning_rate * 0.1).max(0.01);
        }
        
        println!("[AURORAE++] Agent s'adaptant : Exploration → {:.3}, Apprentissage → {:.3}", 
                 self.exploration_rate, self.learning_rate);
    }

    // Nouvelle fonction : Évoluer le réseau neural (augmenter sa complexité)
    pub fn evolve_network(&mut self) {
        self.network_complexity += 1;
        self.evolution_count += 1;
        
        // Mettre à jour le timestamp d'évolution
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        self.last_evolution_timestamp = current_time;
        
        // Ajuster les seuils en fonction de la nouvelle complexité
        self.adaptation_threshold *= 0.9; // Plus exigeant avec l'évolution
        self.evolution_threshold *= 0.85; // Encore plus exigeant pour l'évolution suivante
        
        // Réduire l'exploration à mesure que le réseau évolue
        self.exploration_rate = (self.exploration_rate * 0.9).max(0.05);
        
        println!("[AURORAE++] Évolution #{} : Complexité réseau → {}", 
                 self.evolution_count, self.network_complexity);
        
        // Explorer de nouvelles actions possibles lors d'une évolution
        self.explore_new_actions();
    }

    // Nouvelle fonction : Explorer de nouvelles actions potentielles
    pub fn explore_new_actions(&mut self) {
        // Simuler la découverte de nouvelles actions possibles
        let mut new_actions = Vec::new();
        let mut rng = rand::thread_rng();
        
        // Générer entre 1 et 3 nouvelles actions potentielles
        let num_new_actions = rng.gen_range(1..=3);
        
        for i in 0..num_new_actions {
            let new_action_id = format!("action_evolved_{}", self.actions.len() + i);
            
            // Vérifier si cette action n'existe pas déjà
            if !self.actions.contains(&new_action_id) {
                new_actions.push(new_action_id);
            }
        }
        
        // Ajouter les nouvelles actions à notre répertoire
        for action in &new_actions {
            self.add_new_action_to_q_table(action);
        }
        
        println!("[AURORAE++] {} nouvelles actions découvertes", new_actions.len());
    }

    // Nouvelle fonction : Ajouter une nouvelle action à la Q-table
    pub fn add_new_action_to_q_table(&mut self, action: &str) {
        // Vérifier si l'action existe déjà
        if self.actions.contains(&action.to_string()) {
            return;
        }
        
        // Ajouter l'action à la liste des actions
        self.actions.push(action.to_string());
        
        // Initialiser les entrées Q-table pour cette action avec tous les états connus
        let mut action_map = HashMap::new();
        for state in &self.known_states {
            action_map.insert(state.clone(), 0.0);
        }
        
        // Ajouter la nouvelle action à la Q-table
        self.q_table.insert(action.to_string(), action_map);
        
        println!("[AURORAE++] Nouvelle action ajoutée : {}", action);
    }

    // Nouvelle fonction : Générer une stratégie basée sur l'apprentissage actuel
    pub fn generate_strategy(&mut self) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        // Construire une stratégie à partir des meilleures actions pour chaque état connu
        let mut state_action_map = HashMap::new();
        
        for state in &self.known_states {
            // Trouver la meilleure action pour cet état
            let best_actions = self.find_top_actions(1);
            if !best_actions.is_empty() {
                state_action_map.insert(state.clone(), best_actions[0].clone());
            }
        }
        
        // Ne créer la stratégie que si elle a un nombre minimum d'états
        if state_action_map.len() >= 10 {
            let strategy = Strategy {
                name: format!("strategy_{}", self.strategies.len() + 1),
                state_action_map,
                effectiveness: 0.5, // Commencer avec une efficacité moyenne
                usage_count: 0,
                last_updated: current_time,
                creation_context: format!("Evolution #{}, Performance {:.2}", 
                    self.evolution_count, self.current_episode.performance_score),
            };
            
            self.strategies.push(strategy);
            println!("[AURORAE++] Nouvelle stratégie générée : {}", 
                self.strategies.last().unwrap().name);
        }
    }

    // Nouvelle fonction : Explorer une nouvelle stratégie (variation de la meilleure actuelle)
    pub fn explore_new_strategy(&mut self) {
        if self.strategies.is_empty() {
            // Générer une première stratégie si aucune n'existe
            self.generate_strategy();
            return;
        }
        
        // Trouver la stratégie la plus efficace
        let mut best_strategy_index = 0;
        let mut best_effectiveness = 0.0;
        
        for (i, strategy) in self.strategies.iter().enumerate() {
            if strategy.effectiveness > best_effectiveness {
                best_effectiveness = strategy.effectiveness;
                best_strategy_index = i;
            }
        }
        
        // Créer une variation de cette stratégie
        // Récupérer la meilleure stratégie de manière immuable
let best_strategy = &self.strategies[best_strategy_index];

// Maintenant, ajouter la nouvelle stratégie de manière mutable
// Créez une nouvelle stratégie à partir de best_strategy ou d'une logique similaire
let new_strategy = Strategy {
    name: format!("strategy_{}", self.strategies.len() + 1),
    state_action_map: best_strategy.state_action_map.clone(),
    effectiveness: best_strategy.effectiveness * 0.8,  // Exemple de mutation
    usage_count: 0,
    last_updated: current_time,
    creation_context: format!("Mutation de {} avec {} changements", best_strategy.name, num_mutations),
};

// Ajouter la nouvelle stratégie à la liste
self.strategies.push(new_strategy);

// Utiliser la meilleure stratégie après avoir effectué l'emprunt mutable
        let mut new_state_action_map = best_strategy.state_action_map.clone();
        
        // Modifier quelques états-actions aléatoires (10-30% de mutations)
        let mut rng = rand::thread_rng();
        let num_mutations = (new_state_action_map.len() as f32 * 
            rng.gen_range(0.1..=0.3)).round() as usize;
        
        let states: Vec<String> = new_state_action_map.keys().cloned().collect();
        for _ in 0..num_mutations {
            if states.is_empty() {
                break;
            }
            
            // Sélectionner un état aléatoire
            let state_index = rng.gen_range(0..states.len());
            let state = &states[state_index];
            
            // Sélectionner une action aléatoire différente
            let current_action = new_state_action_map.get(state).unwrap();
            let mut new_actions: Vec<String> = self.actions.iter()
                .filter(|&a| a != current_action)
                .cloned()
                .collect();
            
            if !new_actions.is_empty() {
                let new_action = &new_actions[rng.gen_range(0..new_actions.len())];
                new_state_action_map.insert(state.clone(), new_action.clone());
            }
        }
        
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        // Créer la nouvelle stratégie
        let new_strategy = Strategy {
            name: format!("strategy_{}_{}", best_strategy.name, self.strategies.len() + 1),
            state_action_map: new_state_action_map,
            effectiveness: best_strategy.effectiveness * 0.8, // Légèrement inférieure au départ
            usage_count: 0,
            last_updated: current_time,
            creation_context: format!("Mutation de {} avec {} changements", 
                best_strategy.name, num_mutations),
        };
        
        // Créez une nouvelle stratégie à partir de best_strategy ou d'une logique similaire
let new_strategy = Strategy {
    name: format!("strategy_{}", self.strategies.len() + 1),
    state_action_map: best_strategy.state_action_map.clone(),
    effectiveness: best_strategy.effectiveness * 0.8,  // Exemple de mutation
    usage_count: 0,
    last_updated: current_time,
    creation_context: format!("Mutation de {} avec {} changements", best_strategy.name, num_mutations),
};

// Ajouter la nouvelle stratégie à la liste
self.strategies.push(new_strategy);
        println!("[AURORAE++] Stratégie mutée créée à partir de {}", best_strategy.name);
    }

    // Nouvelle fonction : Processus de "rêve" pour consolider l'apprentissage
    pub fn dream(&mut self) {
        println!("[AURORAE++] Démarrage du cycle de rêve...");
        
        // Sélectionner quelques épisodes de mémoire à long terme pour "rêver"
        let mut rng = rand::thread_rng();
        let num_episodes = (self.long_term_memory.len() / 10).max(1).min(5);
        
        for _ in 0..num_episodes {
            if self.long_term_memory.is_empty() {
                break;
            }
            
            // Sélectionner un épisode aléatoire, mais avec tendance vers les plus performants
            self.long_term_memory.sort_by(|a, b| 
                b.performance_score.partial_cmp(&a.performance_score)
                    .unwrap_or(std::cmp::Ordering::Equal));
            
            let episode_index = (rng.gen::<f32>().powi(2) * self.long_term_memory.len() as f32) as usize;
            // Récupérer un épisode de manière immuable
let episode = &self.long_term_memory[episode_index.min(self.long_term_memory.len() - 1)];

// Utiliser l'épisode pour mettre à jour la Q-table
for i in 0..(episode.action_history.len().min(episode.state_history.len() - 1)) {
    let state = &episode.state_history[i];
    let action = &episode.action_history[i];
    let reward = episode.reward_history[i];
    let next_state = &episode.state_history[i + 1];

    // Modifier légèrement la récompense pour explorer des variations
    let dream_reward = if rng.gen::<f32>() < 0.2 {
        reward * rng.gen_range(0.8..1.2)
    } else {
        reward
    };

    // Mettre à jour la Q-table après avoir terminé avec l'emprunt immuable
    self.update_q_value(action, dream_reward, next_state);
}
            
            // "Rejouer" cet épisode avec des variations pour renforcer l'apprentissage
            for i in 0..(episode.action_history.len().min(episode.state_history.len() - 1)) {
                let state = &episode.state_history[i];
                let action = &episode.action_history[i];
                let reward = episode.reward_history[i];
                let next_state = &episode.state_history[i + 1];
                
                // Modifier légèrement la récompense pour explorer des variations
                let dream_reward = if rng.gen::<f32>() < 0.2 {
                    reward * rng.gen_range(0.8..1.2)
                } else {
                    reward
                };
                
                // Mettre à jour la Q-table avec cette expérience de rêve
                // Utilisons un taux d'apprentissage plus faible pour le rêve
                let original_lr = self.learning_rate;
                self.learning_rate *= 0.3; // Réduire l'impact des rêves
                
                self.state = state.clone(); // Temporairement changer l'état pour la mise à jour
                self.update_q_value(action, dream_reward, next_state);
                
                self.learning_rate = original_lr;
            }
        }
        
        println!("[AURORAE++] Cycle de rêve terminé. {} épisodes rejoués.", num_episodes);
    }

    // Affiche la table Q (inchangé mais avec formatage amélioré)
    pub fn print_q_table(&self) {
        println!("[AURORAE++] Table Q ({} états, {} actions):", 
                 self.known_states.len(), self.actions.len());
        
        // Afficher seulement un échantillon représentatif pour éviter de submerger la sortie
        let mut states: Vec<&String> = self.known_states.iter().collect();
        states.sort();
        
        let sample_size = 5.min(states.len());
        if sample_size < states.len() {
            println!("  (Affichage d'un échantillon de {} états sur {})", sample_size, states.len());
        }
        
        for state in states.iter().take(sample_size) {
            println!("  État: {}", state);
            
            for action in &self.actions {
                let q_value = match self.q_table.get(action) {
                    Some(action_map) => match action_map.get(*state) {
                        Some(value) => *value,
                        None => 0.0,
                    },
                    None => 0.0,
                };
                
                println!("    → {}: {:.3}", action, q_value);
            }
        }
    }
    
    // Nouvelle fonction: Rapport de performance et d'état
    pub fn performance_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("[AURORAE++] Rapport de performance (Agent évolution #{})\n", 
                                 self.evolution_count));
        report.push_str(&format!("-----------------------------------------------\n"));
        
        // Statistiques générales
        report.push_str(&format!("États connus: {}\n", self.known_states.len()));
        report.push_str(&format!("Actions disponibles: {}\n", self.actions.len()));
        report.push_str(&format!("Complexité du réseau: {}\n", self.network_complexity));
        report.push_str(&format!("Stratégies développées: {}\n", self.strategies.len()));
        report.push_str(&format!("Épisodes en mémoire: {}\n", self.long_term_memory.len()));
        
        // Performance actuelle
        if !self.performance_history.is_empty() {
            let last_perf = self.performance_history.last().unwrap().1;
            report.push_str(&format!("\nPerformance actuelle: {:.3}\n", last_perf));
            
            // Tendance de performance (sur les 5 dernières mesures)
            if self.performance_history.len() >= 6 {
                let prev_perf = self.performance_history[self.performance_history.len() - 6].1;
                let trend = last_perf - prev_perf;
                let trend_symbol = if trend > 0.05 {
                    "↑"
                } else if trend < -0.05 {
                    "↓"
                } else {
                    "→"
                };
                
                report.push_str(&format!("Tendance: {} ({:+.3})\n", trend_symbol, trend));
            }
        }
        
        // Paramètres actuels
        report.push_str(&format!("\nParamètres:\n"));
        report.push_str(&format!("  Taux d'apprentissage: {:.3}\n", self.learning_rate));
        report.push_str(&format!("  Taux d'exploration: {:.3}\n", self.exploration_rate));
        report.push_str(&format!("  Facteur de discount: {:.3}\n", self.discount_factor));
        
        // Stratégies les plus efficaces
        if !self.strategies.is_empty() {
            report.push_str(&format!("\nMeilleures stratégies:\n"));
            
            let mut sorted_strategies = self.strategies.clone();
            sorted_strategies.sort_by(|a, b| 
                b.effectiveness.partial_cmp(&a.effectiveness)
                    .unwrap_or(std::cmp::Ordering::Equal));
            
            for (i, strategy) in sorted_strategies.iter().take(3).enumerate() {
                report.push_str(&format!("  {}. {} (eff: {:.3}, utilisations: {})\n",
                                       i+1, strategy.name, strategy.effectiveness, strategy.usage_count));
            }
        }
        
        report
    }
    
    // Sauvegarder l'état de l'agent (sérialisé)
    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        println!("[AURORAE++] Agent sauvegardé dans {}", path);
        Ok(())
    }
    
    // Charger l'état de l'agent
    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let agent: LearningAgent = serde_json::from_str(&json)?;
        println!("[AURORAE++] Agent chargé depuis {}", path);
        Ok(agent)
    }
}
