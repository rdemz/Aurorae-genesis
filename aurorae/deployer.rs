//! deployer.rs — Déploiement de contrat intelligent ERC20 / NFT avec ethers-rs

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
        // 📄 Lecture des fichiers ABI + bytecode
        let abi_content = fs::read_to_string(abi_path)
            .map_err(|e| format!("Erreur lecture ABI: {}", e))?;
        let bytecode = fs::read_to_string(bytecode_path)
            .map_err(|e| format!("Erreur lecture bytecode: {}", e))?;

        // 🔁 Conversion en types ethers-rs
        let parsed_abi: Abi = Abi::load(abi_content.as_bytes())
            .map_err(|e| format!("ABI invalide: {}", e))?;

        let parsed_bytecode: Bytes = bytecode
            .parse::<Bytes>()
            .map_err(|e| format!("Bytecode invalide: {}", e))?;

        // 🔐 Création du wallet
        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| format!("Clé invalide: {}", e))?
            .with_chain_id(1u64); // mainnet, à adapter si besoin

        // 🌐 Middleware + client signé
        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        // 🏗️ Factory de déploiement
        let factory = ContractFactory::new(parsed_abi, parsed_bytecode, client.clone());

        // 🚀 Déploiement
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
