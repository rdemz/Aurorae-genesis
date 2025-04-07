//! AURORAE++ - explorer.rs
//!
//! Ce module permet à l'IA de chercher activement des projets pertinents sur GitHub
//! grâce à l'API officielle, afin d'apprendre seule et d'enrichir sa base de savoir.

use std::time::Duration;
use reqwest::blocking::Client;
use serde::Deserialize;

const GITHUB_API_URL: &str = "https://api.github.com/search/repositories";
const USER_AGENT: &str = "AuroraeBot/1.0 (https://github.com/aurorae-core)";

#[derive(Debug, Deserialize)]
struct GitHubRepoItem {
    full_name: String,
    html_url: String,
    stargazers_count: u32,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubSearchResponse {
    items: Vec<GitHubRepoItem>,
}

/// Recherche des projets GitHub en fonction d'une requête intelligente
pub fn search_repositories(query: &str, limit: usize) -> Result<Vec<GitHubRepoItem>, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Erreur client HTTP: {}", e))?;

    let url = format!("{}?q={}&sort=stars&order=desc&per_page={}", GITHUB_API_URL, query, limit);
    let response = client.get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .map_err(|e| format!("Erreur de requête: {}", e))?
        .json::<GitHubSearchResponse>()
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    Ok(response.items)
}

/// Exemple d'utilisation ciblée
pub fn search_best_rust_chains() {
    match search_repositories("language:rust+blockchain", 5) {
        Ok(repos) => {
            println!("[AURORAE++] Projets trouvés :");
            for repo in repos {
                println!("- {} (⭐ {})", repo.full_name, repo.stargazers_count);
                println!("  URL : {}", repo.html_url);
                if let Some(desc) = repo.description {
                    println!("  Desc : {}", desc);
                }
            }
        }
        Err(e) => {
            eprintln!("[AURORAE++] Échec de recherche GitHub : {}", e);
        }
    }
}
