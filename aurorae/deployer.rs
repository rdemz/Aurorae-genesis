//! deployer.rs — Module de déploiement automatique via ethers-rs

use std::fs;
use std::sync::Arc;
use std::time::Duration;

use ethers::prelude::*;
use ethers::types::Address;
use ethers::signers::LocalWallet;
use ethers::middleware::SignerMiddleware;
use ethers::contract::ContractFactory;

use crate::blockchain_core::HttpProvider;

pub struct Deployer;

impl Deployer {
    pub async fn deploy_contract(
        provider: HttpProvider,
        private_key: &str,
        abi_path: &str,
        bytecode_path: &str,
    ) -> Result<Address, String> {
        let abi = fs::read_to_string(abi_path)
            .map_err(|e| format!("Erreur lecture ABI: {}", e))?;
        let bytecode = fs::read_to_string(bytecode_path)
            .map_err(|e| format!("Erreur lecture bytecode: {}", e))?;

        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| format!("Clé invalide: {}", e))?
            .with_chain_id(1u64); // 🛠️ Mainnet — adapte selon réseau

        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        let factory = ContractFactory::new(
            abi.parse().map_err(|e| format!("ABI invalide: {}", e))?,
            bytecode.parse().map_err(|e| format!("Bytecode invalide: {}", e))?,
            client.clone(),
        );

        let contract = factory
            .deploy(()) // 🛠️ Paramètres si constructeur en attend
            .map_err(|e| format!("Erreur déploiement: {}", e))?
            .confirmations(3)
            .timeout(Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("Erreur exécution: {}", e))?;

        println!(
            "[AURORAE++] ✅ Contrat déployé à l’adresse : {:?}",
            contract.address()
        );

        Ok(contract.address())
    }
}
