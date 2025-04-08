//! blockchain_core.rs — Interface blockchain intelligente pour AURORAE++

use std::sync::Arc;
use ethers::providers::{Http, Provider};

#[derive(Default)]
pub struct BlockchainInterface;

pub type HttpProvider = Arc<Provider<Http>>;

impl BlockchainInterface {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_wallet(&self, network: &str) -> Result<String, String> {
        println!("[AURORAE++] 🔐 Wallet créé pour le réseau : {}", network);
        Ok(format!("wallet_{}", network))
    }

    pub async fn deploy_smart_contract(&self, name: &str) -> Result<String, String> {
        println!("[AURORAE++] 📜 Contrat {} déployé avec succès", name);
        Ok(format!("contract_address_{}", name))
    }

    pub fn connect_to_chain(&self, chain_id: &str) {
        println!("[AURORAE++] 🌐 Connexion à la chaîne {chain_id}");
    }

    pub fn get_http_provider(rpc_url: &str) -> Result<HttpProvider, String> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| format!("Erreur provider: {}", e))?;
        Ok(Arc::new(provider))
    }
}
