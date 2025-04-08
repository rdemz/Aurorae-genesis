use std::collections::HashMap;
use rand::Rng; // Pour générer des valeurs aléatoires

// Définir l'agent d'apprentissage par renforcement
pub struct LearningAgent {
    pub actions: Vec<String>, // Liste des actions possibles
    pub state: String, // L'état actuel du système
    pub q_table: HashMap<String, HashMap<String, f32>>, // Tableau des Q-values pour chaque état et action
    pub learning_rate: f32, // Taux d'apprentissage
    pub discount_factor: f32, // Facteur de discount (pour l'importance des récompenses futures)
    pub exploration_rate: f32, // Taux d'exploration (pour choisir des actions aléatoires)
}

impl LearningAgent {
    // Création d'un nouvel agent d'apprentissage
    pub fn new(actions: Vec<String>, initial_state: &str) -> Self {
        let mut q_table = HashMap::new();
        // Initialiser la table Q avec des valeurs nulles pour chaque combinaison état-action
        for action in &actions {
            let mut action_map = HashMap::new();
            action_map.insert(initial_state.to_string(), 0.0);
            q_table.insert(action.clone(), action_map);
        }

        LearningAgent {
            actions,
            state: initial_state.to_string(),
            q_table,
            learning_rate: 0.1,
            discount_factor: 0.9,
            exploration_rate: 0.1,
        }
    }

    // Choisir une action (avec exploration ou exploitation)
    pub fn choose_action(&mut self) -> String {
        let mut rng = rand::thread_rng();

        // Exploration vs exploitation : on choisit une action aléatoire ou la meilleure action
        if rng.gen::<f32>() < self.exploration_rate {
            // Exploration : choisir une action au hasard
            let action = &self.actions[rng.gen_range(0..self.actions.len())];
            action.to_string()
        } else {
            // Exploitation : choisir l'action avec la meilleure Q-value pour l'état actuel
            let best_action = self.actions.iter()
                .max_by(|a, b| {
                    let a_q_value = self.q_table.get(*a)
                        .and_then(|action_map| action_map.get(&self.state))
                        .unwrap_or(&0.0);
                    let b_q_value = self.q_table.get(*b)
                        .and_then(|action_map| action_map.get(&self.state))
                        .unwrap_or(&0.0);
                    a_q_value.partial_cmp(&b_q_value).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            best_action.to_string()
        }
    }

    // Mettre à jour la Q-value de l'action choisie pour l'état actuel
    pub fn update_q_value(&mut self, action: &str, reward: f32, next_state: &str) {
        // Vérifier si l'action existe dans la table Q
        let current_q_value = self.q_table.entry(action.to_string())
            .or_insert_with(HashMap::new)
            .entry(self.state.clone())
            .or_insert(0.0);

        // Calcul de la nouvelle Q-value
        // D'abord, calculons la valeur maximale des Q-values futures, sans emprunter immuablement pendant l'emprunt mutable
        let max_future_q = self.actions.iter()
            .filter_map(|a| {
                self.q_table.get(a).and_then(|action_map| action_map.get(next_state))
            }) // Récupérer la Q-value pour le prochain état
            .cloned() // Cloner les valeurs
            .fold(0.0, f32::max); // Trouver la valeur maximale

        // Mise à jour de la Q-value
        let new_q_value = *current_q_value + self.learning_rate * (reward + self.discount_factor * max_future_q - *current_q_value);
        *current_q_value = new_q_value;
    }

    // Appliquer la logique d'apprentissage pour chaque cycle
    pub fn learn(&mut self, reward: f32, next_state: &str) {
        let action = self.choose_action();
        self.update_q_value(&action, reward, next_state);
        self.state = next_state.to_string(); // Mettre à jour l'état après l'action
    }

    // Affichage de la Q-table pour déboguer et observer l'apprentissage
    pub fn print_q_table(&self) {
        println!("[AURORAE++] Q-table:");
        for (action, action_map) in &self.q_table {
            for (state, q_value) in action_map {
                println!("Action: {}, State: {}, Q-value: {}", action, state, q_value);
            }
        }
    }
}
