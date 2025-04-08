//! deployer.rs — Déploiement de contrat intelligent

use std::fs;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use ethers::prelude::*;
use ethers::types::Address;
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
        let abi_content = fs::read_to_string(abi_path)
            .map_err(|e| format!("Erreur lecture ABI: {}", e))?;
        let bytecode = fs::read_to_string(bytecode_path)
            .map_err(|e| format!("Erreur lecture bytecode: {}", e))?;

        let parsed_abi: Abi = Abi::load(abi_content.as_bytes())
            .map_err(|e| format!("ABI invalide: {}", e))?;

        let wallet: LocalWallet = private_key
            .parse()
            .map_err(|e| format!("Clé invalide: {}", e))?
            .with_chain_id(1u64);

        let client = SignerMiddleware::new(provider.clone(), wallet);
        let client = Arc::new(client);

        let factory = ContractFactory::new(
            parsed_abi,
            bytecode.parse().map_err(|e| format!("Bytecode invalide: {}", e))?,
            client.clone(),
        );

        let contract = factory
            .deploy(())?
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
