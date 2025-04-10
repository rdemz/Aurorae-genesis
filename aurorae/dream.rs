
use chrono::Utc;
use std::collections::VecDeque;
use uuid::Uuid;
use rand::Rng;
use reqwest::Error;
use serde::Deserialize;
use std::fs;
use std::path::Path;

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
                "Inspiré par le projet GitHub: {}
Description: {}
URL: {}",
                name, description, html_url
            );
            return Ok(inspiration);
        }
    }

    Ok("Aucune inspiration trouvée sur GitHub".to_string())
}

// Fonction pour charger l'inspiration depuis le répertoire local
fn load_local_inspiration(path: &str) -> String {
    let path = Path::new(path);

    if path.exists() && path.is_dir() {
        // Lire les fichiers du répertoire
        match fs::read_dir(path) {
            Ok(entries) => {
                let mut inspirations = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let entry_path = entry.path();
                            if entry_path.is_file() {
                                let file_name = entry_path.file_name().unwrap().to_str().unwrap();
                                inspirations.push(format!("Fichier trouvé: {}", file_name));
                            }
                        },
                        Err(e) => eprintln!("Error reading entry: {}", e),
                    }
                }
                if !inspirations.is_empty() {
                    return inspirations.join("
");
                }
            },
            Err(e) => eprintln!("Error reading directory: {}", e),
        }
    }

    "Aucune inspiration locale trouvée.".to_string()
}

// Structure représentant un rêve
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

    // Méthode pour imaginer un nouveau rêve
    pub async fn imagine(&mut self, title: &str, description: &str, image_url: &str) {
        let mut rng = rand::thread_rng();

        // Récupérer l'inspiration depuis GitHub
        let github_inspiration = fetch_github_inspiration().await.unwrap_or_else(|_| "Aucune inspiration GitHub trouvée.".to_string());
        
        // Récupérer l'inspiration depuis le répertoire local
        let local_inspiration = load_local_inspiration(r"C:\Users\admin\inspiration");

        // Fusionner les inspirations
        let combined_inspiration = format!("{}

Inspiration locale:
{}", github_inspiration, local_inspiration);

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
            external_inspiration: combined_inspiration, // Ajouter l'inspiration combinée
        };

        println!("[AURORAE++] 💭 Nouveau rêve: {}", title);
        println!("[AURORAE++] 📖 Inspiration combinée:
{}", dream.external_inspiration); // Afficher l'inspiration combinée

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
}
