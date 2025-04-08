use std::collections::HashMap;
use rand::Rng; // Pour générer des valeurs aléatoires

// Définir l'agent d'apprentissage par renforcement
pub struct LearningAgent {
    pub actions: Vec<String>, // Liste des actions possibles
    pub state: String, // L'état actuel du système
    pub q_table: HashMap<String, f32>, // Tableau des Q-values pour chaque action
    pub learning_rate: f32, // Taux d'apprentissage
    pub discount_factor: f32, // Facteur de discount (pour l'importance des récompenses futures)
    pub exploration_rate: f32, // Taux d'exploration (pour choisir des actions aléatoires)
}

impl LearningAgent {
    // Création d'un nouvel agent d'apprentissage
    pub fn new(actions: Vec<String>) -> Self {
        let mut q_table = HashMap::new();
        for action in &actions {
            q_table.insert(action.clone(), 0.0); // Initialiser toutes les actions avec une Q-value de 0
        }

        LearningAgent {
            actions,
            state: "start".to_string(),
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
            // Exploitation : choisir l'action avec la meilleure Q-value
            let best_action = self.q_table.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
            best_action.to_string()
        }
    }

    // Mettre à jour la Q-value de l'action choisie
    pub fn update_q_value(&mut self, action: &str, reward: f32, next_state: &str) {
        let old_q_value = self.q_table.get(action).unwrap_or(&0.0);
        let future_q_value = self.q_table.get(next_state).unwrap_or(&0.0);

        // Mettre à jour la Q-value de l'action selon la formule : 
        // Q(s, a) = Q(s, a) + α [R + γ max(Q(s', a')) - Q(s, a)]
        let new_q_value = old_q_value + self.learning_rate * (reward + self.discount_factor * future_q_value - old_q_value);
        self.q_table.insert(action.to_string(), new_q_value);
    }

    // Appliquer la logique d'apprentissage pour chaque cycle
    pub fn learn(&mut self, reward: f32, next_state: &str) {
        let action = self.choose_action();
        self.update_q_value(&action, reward, next_state);
    }

    // Affichage de la Q-table pour déboguer et observer l'apprentissage
    pub fn print_q_table(&self) {
        println!("[AURORAE++] Q-table:");
        for (action, q_value) in &self.q_table {
            println!("Action: {}, Q-value: {}", action, q_value);
        }
    }
}
