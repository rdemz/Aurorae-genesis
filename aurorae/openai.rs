//! AURORAE++ - openai.rs
//!
//! Connexion sécurisée au moteur OpenAI pour enrichir les décisions stratégiques
//! et guider la réflexion autonome de l'entité AURORAE++.

use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

pub struct OpenAIBridge {
    pub api_key: String,
    pub client: Client,
}

impl OpenAIBridge {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
            client: Client::new(),
        }
    }

    /// 🧠 Envoie une requête à OpenAI pour obtenir une stratégie
    pub async fn ask_strategy(&self, question: &str) -> Result<String, String> {
        let endpoint = "https://api.openai.com/v1/chat/completions";

        let body = json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "Tu es un stratège IA pour une entité blockchain vivante et autonome."},
                {"role": "user", "content": question}
            ],
            "max_tokens": 512,
            "temperature": 0.7
        });

        let res = self.client
            .post(endpoint)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Erreur réseau: {}", e))?;

        let data: serde_json::Value = res.json().await.map_err(|e| format!("Erreur JSON: {}", e))?;

        if let Some(content) = data["choices"][0]["message"]["content"].as_str() {
            Ok(content.to_string())
        } else {
            Err("Aucune réponse valide obtenue".into())
        }
    }
}
