//! deployer.rs — Déploiement de contrats EVM avec ethers-rs 2.0.14

use std::fs;
use std::str::FromStr;
use std::sync::Arc;

use ethers::prelude::*;
use ethers::types::{Address, Bytes};
use ethers::contract::ContractFactory;
use ethers::middleware::SignerMiddleware;
use ethers::signers::LocalWallet;
use ethers::abi::Abi;

use crate::blockchain_core::HttpProvider;

pub struct Deployer;

impl Deployer {
    pub async fn deploy_contract(
        provider: HttpProvider,
        private_key: &str,
        abi_path: &str,
        bytecode_path: &str,
    ) -> Result<Address, String> {
        // 📄 Lecture de l’ABI et du bytecode
        let abi_content = fs::read_to_string(abi_path)
            .map_err(|e| format!("Erreur lecture ABI: {}", e))?;
        let bytecode = fs::read_to_string(bytecode_path)
            .map_err(|e| format!("Erreur lecture bytecode: {}", e))?;

        // ✅ Chargement de l’ABI
        let parsed_abi: Abi = Abi::load(abi_content.as_bytes())
            .map_err(|e| format!("ABI invalide: {}", e))?;

        // ✅ Parsing du bytecode → Bytes (type ethers)
        let parsed_bytecode = bytecode
            .parse::<Bytes>()
            .map_err(|e| format!("Bytecode invalide: {}", e))?;

        // 🔐 Construction du wallet
        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| format!("Clé privée invalide: {}", e))?
            .with_chain_id(1u64); // à adapter selon le réseau

        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        // 🏗️ Factory pour déploiement
        let factory = ContractFactory::new(parsed_abi, parsed_bytecode, client.clone());

        // 🚀 Déploiement du contrat
        let contract = factory
            .deploy(())
            .map_err(|e| format!("Erreur création déploiement: {}", e))?
            .confirmations(3)
            .send()
            .await
            .map_err(|e| format!("Erreur déploiement: {}", e))?;

        println!(
            "[AURORAE++] ✅ Contrat déployé à l’adresse : {:?}",
            contract.address()
        );

        Ok(contract.address())
    }
}
