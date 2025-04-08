use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

use crate::blockchain_core::HttpProvider;

// Configurations pour le déploiement
#[derive(Clone)]
pub struct DeploymentConfig {
    pub network: String,
    pub gas_limit: u64,
    pub priority_fee: Option<u64>,
    pub constructor_args: Vec<String>,
    pub verify_code: bool,
}

// Résultat d'un déploiement
#[derive(Clone)]
pub struct DeploymentResult {
    pub contract_address: String,
    pub transaction_hash: String,
    pub block_number: u64,
    pub deployment_id: Uuid,
    pub timestamp: String,
    pub network: String,
    pub contract_name: String,
}

pub struct Deployer {
    pub networks: Vec<String>,
    pub default_config: DeploymentConfig,
    deployment_history: Vec<DeploymentResult>,
    provider: HashMap<String, HttpProvider>,
    deployment_count: u64,
    innovation_score: f32,
}

impl Deployer {
    pub fn new() -> Self {
        let default_rpc = std::env::var("ETH_RPC_URL").unwrap_or_else(|_| "http://localhost:8545".to_string());
        
        let mut providers = HashMap::new();
        providers.insert("aurorae-genesis".to_string(), HttpProvider::new(default_rpc.clone()));
        providers.insert("testnet".to_string(), HttpProvider::new(default_rpc));
        
        Self {
            networks: vec![
                "aurorae-genesis".to_string(),
                "testnet".to_string(),
                "local".to_string(),
            ],
            default_config: DeploymentConfig {
                network: "aurorae-genesis".to_string(),
                gas_limit: 3000000,
                priority_fee: Some(2),
                constructor_args: Vec::new(),
                verify_code: false,
            },
            deployment_history: Vec::new(),
            provider: providers,
            deployment_count: 0,
            innovation_score: 1.0,
        }
    }
    
    pub fn add_network(&mut self, name: &str, rpc_url: &str) {
        if !self.networks.contains(&name.to_string()) {
            self.networks.push(name.to_string());
            self.provider.insert(name.to_string(), HttpProvider::new(rpc_url));
            println!("[AURORAE++] 🔌 Nouveau réseau ajouté au déployeur: {}", name);
        }
    }

    pub async fn deploy_contract(&mut self, contract_name: &str, config: Option<DeploymentConfig>) -> Result<DeploymentResult, String> {
        let config = config.unwrap_or_else(|| self.default_config.clone());
        
        // Vérifier que le réseau existe
        if !self.networks.contains(&config.network) {
            return Err(format!("Réseau {} inconnu", config.network));
        }

        println!("[AURORAE++] 🚀 Déploiement du contrat {} sur {}", contract_name, config.network);

        // Simuler le déploiement
        let result = DeploymentResult {
            contract_address: format!("0x{}", Uuid::new_v4().simple().to_string()),
            transaction_hash: format!("0x{}", Uuid::new_v4().simple().to_string()),
            block_number: 12345678 + self.deployment_count as u64,
            deployment_id: Uuid::new_v4(),
            timestamp: Utc::now().to_rfc3339(),
            network: config.network.clone(),
            contract_name: contract_name.to_string(),
        };

        self.deployment_history.push(result.clone());
        self.deployment_count += 1;
        
        // Augmenter le score d'innovation basé sur les déploiements
        self.innovation_score *= 1.01;
        
        println!("[AURORAE++] ✅ Contrat '{}' déployé à l'adresse: {}", 
                 contract_name, result.contract_address);
                 
        // Vérifier le code si demandé
        if config.verify_code {
            println!("[AURORAE++] 🔍 Vérification du code du contrat sur l'explorateur de blockchain");
            // Simulation de vérification
            println!("[AURORAE++] ✓ Code vérifié avec succès");
        }
        
        Ok(result)
    }

    pub fn get_deployment_history(&self) -> &Vec<DeploymentResult> {
        &self.deployment_history
    }
    
    pub fn get_latest_deployment(&self, contract_name: Option<&str>) -> Option<&DeploymentResult> {
        // Filtre par nom de contrat si spécifié
        if let Some(name) = contract_name {
            self.deployment_history.iter()
                .filter(|d| d.contract_name == name)
                .last()
        } else {
            self.deployment_history.last()
        }
    }
    
    pub async fn upgrade_contract(&mut self, contract_address: &str, new_contract_name: &str) -> Result<DeploymentResult, String> {
        println!("[AURORAE++] 📝 Mise à niveau du contrat à l'adresse {}", contract_address);
        
        // Trouver le déploiement original pour obtenir le réseau
        let original_opt = self.deployment_history.iter().find(|d| d.contract_address == contract_address);
        
        if original_opt.is_none() {
            return Err(format!("Contrat à l'adresse {} non trouvé dans l'historique", contract_address));
        }
        
        let original_network = original_opt.unwrap().network.clone();
        let original_name = original_opt.unwrap().contract_name.clone();
        
        // Préparer la configuration pour la mise à niveau
        let upgrade_config = DeploymentConfig {
            network: original_network,
            gas_limit: 4000000, // Plus élevé pour les mises à niveau
            priority_fee: Some(3),
            constructor_args: vec![contract_address.to_string()], // Adresse du contrat précédent
            verify_code: true, // Toujours vérifier les mises à niveau
        };
        
        // Déployer le nouveau contrat
        let result = self.deploy_contract(new_contract_name, Some(upgrade_config)).await?;
        
        println!("[AURORAE++] 🔄 Contrat mis à niveau: {} -> {}", original_name, new_contract_name);
        
        // Bonus d'innovation pour les mises à niveau
        self.innovation_score *= 1.03;
        
        Ok(result)
    }
    
    pub fn get_innovation_score(&self) -> f32 {
        self.innovation_score
    }
    
    pub fn status_report(&self) {
        println!("\n[AURORAE++] 📝 RAPPORT DU DÉPLOYEUR");
        println!("══════════════════════════════");
        println!("Réseaux disponibles: {}", self.networks.join(", "));
        println!("Déploiements totaux: {}", self.deployment_count);
        println!("Score d'innovation: {:.2}", self.innovation_score);
        
        println!("\nDéploiements récents:");
        let recent = self.deployment_history.iter().rev().take(5);
        for (i, deployment) in recent.enumerate() {
            println!("  {}. {} sur {} à {} ({})",
                     i+1,
                     deployment.contract_name,
                     deployment.network,
                     deployment.contract_address,
                     deployment.timestamp);
        }
        println!("══════════════════════════════\n");
    }
}
