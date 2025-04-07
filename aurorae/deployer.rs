//! AURORAE++ - deployer.rs
//!
//! Ce module permet à l’IA de déployer automatiquement les tokens qu’elle crée
//! sur des blockchains EVM compatibles (Ethereum, Arbitrum, BNB, etc.) via Web3.
//! Utilise `ethers-rs` pour signer, envoyer et suivre les transactions réelles.

use ethers::prelude::*;
use ethers::utils::parse_units;
use std::sync::Arc;
use std::time::Duration;
use std::fs;
use std::path::Path;

// 🔐 Remplace par ta vraie clé privée de déploiement (wallet fondateur)
const DEPLOYER_PRIVATE_KEY: &str = "0xINSERT_YOUR_PRIVATE_KEY_HERE";
const RPC_URL: &str = "https://rpc.ankr.com/eth"; // ou arbitrum, polygon, etc.

/// Gère la compilation et le déploiement d’un token ERC20 généré par AURORAE++
pub async fn deploy_erc20(name: &str, symbol: &str, supply: u64, decimals: u8) -> Result<Address, String> {
    // Charger ABI + bytecode compilés (doit exister dans ./output/auroraium_erc20.json)
    let path = Path::new("./output/auroraium_erc20.json");
    if !path.exists() {
        return Err("⚠️ Fichier ABI + bytecode introuvable (auroraium_erc20.json)".into());
    }
    let file = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let artifact: serde_json::Value = serde_json::from_str(&file).map_err(|e| e.to_string())?;

    let abi = artifact["abi"].clone();
    let bytecode = artifact["bytecode"].as_str().ok_or("Bytecode manquant")?.to_string();
    let abi: Abi = serde_json::from_value(abi).map_err(|e| e.to_string())?;

    let client = connect_wallet().await?;
    let factory = ContractFactory::new(abi, bytecode.parse().unwrap(), client.clone());

    let total_supply = parse_units(supply, decimals.into()).map_err(|e| e.to_string())?;

    let deploy = factory
        .deploy((name.to_string(), symbol.to_string(), total_supply))
        .map_err(|e| e.to_string())?
        .legacy();

    let contract = deploy.send().await.map_err(|e| e.to_string())?;
    let addr = contract.address();
    println!("[AURORAE++] 🛰️ TOKEN DÉPLOYÉ SUR LA BLOCKCHAIN : {:?}", addr);

    Ok(addr)
}

/// Initialise un wallet connecté au réseau EVM\async fn connect_wallet() -> Result<Arc<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>, String> {
    let provider = Provider::<Http>::try_from(RPC_URL)
        .map_err(|e| format!("Provider error: {}", e))?
        .interval(Duration::from_millis(6000));

    let wallet: LocalWallet = DEPLOYER_PRIVATE_KEY
        .parse()
        .map_err(|e| format!("Wallet error: {}", e))?;

    let chain_id = provider.get_chainid().await.map_err(|e| e.to_string())?.as_u64();
    let wallet = wallet.with_chain_id(chain_id);

    Ok(Arc::new(SignerMiddleware::new(provider, wallet)))
}
