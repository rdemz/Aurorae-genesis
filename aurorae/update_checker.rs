extern crate reqwest;
extern crate serde_json;
use std::error::Error;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GitHubRelease {
    pub tag_name: String, // Dernière version du dépôt
    pub name: String,     // Nom de la version
    pub body: Option<String>, // Notes de mise à jour
    pub html_url: String, // URL de la release
}

#[derive(Debug)]
pub struct UpdateChecker {
    pub repo_owner: String, // Propriétaire du repo GitHub
    pub repo_name: String,  // Nom du repo GitHub
    pub current_version: String, // Version actuelle
}

impl UpdateChecker {
    // Crée un nouvel UpdateChecker
    pub fn new(repo_owner: &str, repo_name: &str, current_version: &str) -> Self {
        UpdateChecker {
            repo_owner: repo_owner.to_string(),
            repo_name: repo_name.to_string(),
            current_version: current_version.to_string(),
        }
    }

    // Fonction pour vérifier la dernière version disponible sur GitHub
    pub fn check_for_updates(&self) -> Result<(), Box<dyn Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/releases/latest",
            self.repo_owner
        );

        // Utilisation de reqwest pour envoyer une requête GET
        let client = Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "Aurorae++ Update Checker")
            .send()?;

        // Vérification du statut de la requête
        if !response.status().is_success() {
            return Err("Erreur lors de la récupération des informations de mise à jour.".into());
        }

        // Parsing de la réponse JSON
        let release: GitHubRelease = response.json()?;

        // Comparer la version distante avec la version locale
        if release.tag_name != self.current_version {
            println!(
                "[AURORAE++] Nouvelle version disponible : {} (Votre version : {})",
                release.tag_name, self.current_version
            );
            println!("Détails : {}\n", release.body.unwrap_or("Pas de détails.".to_string()));
            println!("Mise à jour disponible sur : {}", release.html_url);
        } else {
            println!("[AURORAE++] Vous utilisez la dernière version !");
        }

        Ok(())
    }
}

