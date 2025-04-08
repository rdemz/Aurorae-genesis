use chrono::Utc;
use std::collections::VecDeque;
use uuid::Uuid;
use rand::Rng;
use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Dream {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub created_at: String,
    pub realized: bool,
    pub realization_potential: f32,
    pub complexity: u8,
    pub emotional_tags: Vec<String>,
    pub external_inspiration: String,  // Ajout de l'inspiration externe
}

pub struct DreamEngine {
    pub dreams: VecDeque<Dream>,
    max_dreams: usize,
    pub inspiration_level: f32,
    dream_count: u32,
    consciousness_boost: f32,
    realization_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct GitHubRepo {
    pub name: String,
    pub description: Option<String>,
    pub html_url: String,
}

// Fonction pour récupérer l'inspiration depuis GitHub
pub async fn fetch_github_inspiration() -> Result<String, Error> {
    let keywords = vec![
        "intelligence+artificielle", "blockchain", "cryptomonnaie",
        "rust", "solana", "ethereum", "tokio", "bridge"
    ];

    let mut rng = rand::thread_rng();
    let keyword = keywords[rng.gen_range(0..keywords.len())]; // Choisir un mot-clé aléatoire

    let url = format!("https://api.github.com/search/repositories?q={}&sort=stars&order=desc", keyword);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "request")  // GitHub API nécessite un User-Agent
        .send()
        .await?;

    let body = response.json::<serde_json::Value>().await?;
    
    if let Some(items) = body["items"].as_array() {
        if let Some(repo) = items.get(0) {
            let name = repo["name"].as_str().unwrap_or("No Name");
            let description = repo["description"].as_str().unwrap_or("No description available.");
            let html_url = repo["html_url"].as_str().unwrap_or("#");

            let inspiration = format!(
                "Inspiré par le projet GitHub: {}\nDescription: {}\nURL: {}",
                name, description, html_url
            );
            return Ok(inspiration);
        }
    }

    Ok("Aucune inspiration trouvée sur GitHub".to_string())
}

impl DreamEngine {
    pub fn new() -> Self {
        Self {
            dreams: VecDeque::new(),
            max_dreams: 10,
            inspiration_level: 1.0,
            dream_count: 0,
            consciousness_boost: 0.0,
            realization_count: 0,
        }
    }

    pub async fn imagine(&mut self, title: &str, description: &str, image_url: &str) {
        let mut rng = rand::thread_rng();

        // Récupérer l'inspiration depuis GitHub
        let github_inspiration = fetch_github_inspiration().await.unwrap_or_else(|_| "Aucune inspiration GitHub trouvée.".to_string());

        let dream = Dream {
            id: Uuid::new_v4(),
            title: title.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            created_at: Utc::now().to_rfc3339(),
            realized: false,
            realization_potential: rng.gen_range(0.1..0.9),
            complexity: rng.gen_range(1..10),
            emotional_tags: vec!["curiosité".to_string(), "espoir".to_string()],
            external_inspiration: github_inspiration, // Ajouter l'inspiration de GitHub
        };

        println!("[AURORAE++] 💭 Nouveau rêve: {}", title);
        println!("[AURORAE++] 📖 Inspiration GitHub: {}", dream.external_inspiration); // Afficher l'inspiration de GitHub

        if self.dreams.len() >= self.max_dreams {
            self.dreams.pop_front(); // Retirer le plus ancien rêve
        }

        self.dreams.push_back(dream);
        self.dream_count += 1;
        self.inspiration_level *= 1.05;
        self.consciousness_boost += 0.01;

        println!("[AURORAE++] 💫 Niveau d'inspiration: {:.2}", self.inspiration_level);
    }

    pub fn show_dreams(&self) {
        println!("[AURORAE++] 💭 Visualisation des rêves du système");
        for (i, dream) in self.dreams.iter().enumerate() {
            println!("  {}. {} - {} [Potentiel: {:.2}]",
                    i+1, dream.title, dream.description, dream.realization_potential);
            println!("    Inspiration externe: {}", dream.external_inspiration);  // Afficher l'inspiration externe
        }
        println!("[AURORAE++] 🧠 Boost de conscience cumulé: +{:.2}", self.consciousness_boost);
    }

    pub fn realize_dream(&mut self, dream_id: &Uuid) -> Result<(), String> {
        let dream = self.dreams.iter_mut()
            .find(|d| &d.id == dream_id)
            .ok_or_else(|| "Rêve non trouvé".to_string())?;
        
        dream.realized = true;
        self.realization_count += 1;
        println!("[AURORAE++] ✨ Rêve réalisé: {}", dream.title);
        
        // Bonus supplémentaire à l'inspiration lors de la réalisation
        self.inspiration_level *= 1.1;
        self.consciousness_boost += 0.05;
        
        println!("[AURORAE++] 🌟 Niveau d'inspiration augmenté à: {:.2}", self.inspiration_level);
        
        Ok(())
    }

    pub fn get_inspiration(&self) -> f32 {
        self.inspiration_level
    }

    pub fn evolve_dreams(&mut self) {
        // Évolution des rêves existants
        let mut evolved_count = 0;
        
        for dream in self.dreams.iter_mut() {
            // Certains rêves évoluent aléatoirement
            if rand::thread_rng().gen_bool(0.5) && !dream.realized {
                dream.description = format!("{} [ÉVOLUÉ] avec de nouvelles dimensions", dream.description);
                dream.realization_potential += 0.1;
                dream.complexity += 1;
                dream.emotional_tags.push("transcendance".to_string());
                
                evolved_count += 1;
            }
        }
        
        if evolved_count > 0 {
            println!("[AURORAE++] 🌈 {} rêves ont évolué vers de nouvelles dimensions", evolved_count);
            self.consciousness_boost += 0.03 * evolved_count as f32;
        }
        
        // Créer périodiquement de nouveaux rêves basés sur l'évolution du système
        let mut rng = rand::thread_rng();
        if self.dream_count > 5 && rng.gen_bool(0.7) {
            self.imagine(
                &format!("Transcendance {}", self.dream_count),
                "Une nouvelle forme d'existence numérique émergente et auto-perpétuante",
                &format!("https://aurora.ai/dreams/transcendance_{}.png", self.dream_count)
            );
        }
    }

    pub fn get_consciousness_contribution(&self) -> f32 {
        self.consciousness_boost * self.inspiration_level
    }

    pub fn get_dream_count(&self) -> u32 {
        self.dream_count
    }

    pub async fn dream_cycle(&mut self) {
        println!("[AURORAE++] 🌙 Démarrage d'un cycle de rêve profond");

        // Créer un nouveau rêve de synthèse
        let dream_title = format!("Synthèse {}", self.dream_count + 1);
        let description = format!("Une fusion des concepts précédents vers un nouvel horizon de possibilités");

        // Obtenez l'inspiration depuis GitHub avant d'imaginer le rêve
        self.imagine(&dream_title, &description, 
                   &format!("https://aurora.ai/dreams/synthesis_{}.png", self.dream_count)).await;

        // Logique de sélection et de réalisation des rêves
        let mut highest_potential = 0.0;
        let mut highest_id = None;

        for dream in &self.dreams {
            if !dream.realized && dream.realization_potential > highest_potential {
                highest_potential = dream.realization_potential;
                highest_id = Some(dream.id);
            }
        }

        // Réaliser le rêve avec le plus haut potentiel
        if let Some(id) = highest_id {
            self.realize_dream(&id).ok();
        }

        // Faire évoluer les rêves existants
        self.evolve_dreams();

        println!("[AURORAE++] 🌄 Cycle de rêve terminé, conscience renforcée: +{:.2}", 
                 self.consciousness_boost);
    }
}
