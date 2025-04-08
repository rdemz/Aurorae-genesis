//! deployer.rs — Module de déploiement automatique on-chain (ERC20, NFT, etc.)

use crate::blockchain_core::HttpProvider;
use ethers::prelude::*;
use std::sync::Arc;
use std::fs;
use std::time::Duration;

pub struct Deployer;

impl Deployer {
    pub async fn deploy_contract(
        provider: HttpProvider,
        private_key: &str,
        abi_path: &str,
        bytecode_path: &str,
    ) -> Result<Address, String> {
        let abi = fs::read_to_string(abi_path).map_err(|e| format!("Erreur lecture ABI: {}", e))?;
        let bytecode = fs::read_to_string(bytecode_path)
            .map_err(|e| format!("Erreur lecture bytecode: {}", e))?;

        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| format!("Clé invalide: {}", e))?
            .with_chain_id(1u64); // Mainnet, à adapter

        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        let factory = ContractFactory::new(
            abi.parse().map_err(|e| format!("ABI invalide: {}", e))?,
            bytecode.parse().map_err(|e| format!("Bytecode invalide: {}", e))?,
            client.clone(),
        );

        let contract = factory
            .deploy(()) // paramètres de constructeur si nécessaire
            .map_err(|e| format!("Erreur déploiement: {}", e))?
            .confirmations(3)
            .timeout(Duration::from_secs(60))
            .send()
            .await
            .map_err(|e| format!("Erreur exécution: {}", e))?;

        println!("[AURORAE++] ✅ Contrat déployé à l’adresse : {:?}", contract.address());

        Ok(contract.address())
    }
}
