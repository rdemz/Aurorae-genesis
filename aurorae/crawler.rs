//! AURORAE++ - crawler.rs
//!
//! Moteur d'exploration GitHub/Web. Permet à l'IA de télécharger des dépôts réels depuis Internet pour les analyser et apprendre.
//! Utilise la connexion Internet locale de l'utilisateur.

use std::process::Command;
use std::path::Path;
use std::fs;

/// Configuration du chemin d'extraction locale
default const FEED_PATH: &str = "./github_feed";

/// Clone un dépôt GitHub donné vers le dossier local `github_feed/<nom>`
pub fn clone_repo(repo_url: &str) -> Result<(), String> {
    let repo_name = extract_repo_name(repo_url)?;
    let target_dir = format!("{}/{}", FEED_PATH, repo_name);

    if Path::new(&target_dir).exists() {
        println!("[AURORAE++] Dépôt déjà présent localement: {}", target_dir);
        return Ok(());
    }

    println!("[AURORAE++] Clonage de {} vers {}...", repo_url, target_dir);
    let status = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .arg(&target_dir)
        .status()
        .map_err(|e| format!("Erreur de lancement git: {}", e))?;

    if status.success() {
        println!("[AURORAE++] Dépôt cloné avec succès.");
        Ok(())
    } else {
        Err("Échec du clonage Git".to_string())
    }
}

/// Extrait le nom d’un dépôt depuis son URL GitHub
fn extract_repo_name(repo_url: &str) -> Result<String, String> {
    let parts: Vec<&str> = repo_url.rsplit('/').collect();
    if let Some(name) = parts.get(0) {
        Ok(name.replace(".git", ""))
    } else {
        Err("URL GitHub invalide".to_string())
    }
}

/// Nettoie le répertoire d’apprentissage local
pub fn clear_feed() -> Result<(), String> {
    if Path::new(FEED_PATH).exists() {
        fs::remove_dir_all(FEED_PATH).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(FEED_PATH).map_err(|e| e.to_string())?;
    println!("[AURORAE++] Dossier github_feed réinitialisé.");
    Ok(())
}
