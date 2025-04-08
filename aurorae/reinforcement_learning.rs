use std::collections::HashMap;
use rand::Rng;

pub struct LearningAgent {
    pub actions: Vec<String>, // Liste des actions possibles
    pub state: String, // L'état actuel du système
    pub q_table: HashMap<String, HashMap<String, f32>>, // Tableau des Q-values pour chaque état et action
    pub learning_rate: f32, // Taux d'apprentissage
    pub discount_factor: f32, // Facteur de discount (pour l'importance des récompenses futures)
    pub exploration_rate: f32, // Taux d'exploration (pour choisir des actions aléatoires)
}

impl LearningAgent {
    pub fn new(actions: Vec<String>, initial_state: &str) -> Self {
        let mut q_table = HashMap::new();
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

    pub fn choose_action(&mut self) -> String {
        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < self.exploration_rate {
            let action = &self.actions[rng.gen_range(0..self.actions.len())];
            action.to_string()
        } else {
            let best_action = self.actions.iter()
                .max_by(|a, b| {
                    // Récupérer les valeurs de la q_table de manière isolée
                    let a_q_value = self.q_table.get(*a)
                        .map(|action_map| action_map.get(&self.state).unwrap_or(&0.0))
                        .unwrap_or(&0.0);
                    let b_q_value = self.q_table.get(*b)
                        .map(|action_map| action_map.get(&self.state).unwrap_or(&0.0))
                        .unwrap_or(&0.0);

                    a_q_value.partial_cmp(&b_q_value).unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            best_action.to_string()
        }
    }

    pub fn update_q_value(&mut self, action: &str, reward: f32, next_state: &str) {
        // Calculer d'abord la valeur Q maximale pour le prochain état
        // en collectant les valeurs dans un vecteur temporaire
        let mut max_future_q = 0.0;
        
        // Collecter toutes les Q-values pour le prochain état
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
        if !future_q_values.is_empty() {
            max_future_q = *future_q_values.iter().max_by(|a, b| 
                a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
            ).unwrap_or(&0.0);
        }

        // Maintenant, mettre à jour la valeur Q actuelle
        let current_q_value = self.q_table
            .entry(action.to_string())
            .or_insert_with(HashMap::new)
            .entry(self.state.clone())
            .or_insert(0.0);

        // Calculer la nouvelle Q-value
        let new_q_value = *current_q_value + self.learning_rate * (reward + self.discount_factor * max_future_q - *current_q_value);
        *current_q_value = new_q_value;
    }

    pub fn learn(&mut self, reward: f32, next_state: &str) {
        let action = self.choose_action();
        self.update_q_value(&action, reward, next_state);
        self.state = next_state.to_string();
    }

    pub fn print_q_table(&self) {
        println!("[AURORAE++] Q-table:");
        for (action, action_map) in &self.q_table {
            for (state, q_value) in action_map {
                println!("Action: {}, State: {}, Q-value: {}", action, state, q_value);
            }
        }
    }
}
