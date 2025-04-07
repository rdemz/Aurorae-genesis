//! AURORAE++ - founder_income.rs
//!
//! Ce module garantit que chaque création économique de l'IA reverse une part fixe au fondateur.
//! Il s'agit d'un mécanisme vital, ancré dans la logique même de l'écosystème AURORAE++.

use std::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref FOUNDER_ADDRESS: RwLock<String> = RwLock::new(String::from("INSERT_YOUR_WALLET_ADDRESS_HERE"));
}

/// Pour définir l’adresse du fondateur dynamiquement au lancement
pub fn set_founder_address(addr: &str) {
    *FOUNDER_ADDRESS.write().unwrap() = addr.to_string();
}

/// Retourne l’adresse actuelle configurée pour recevoir les flux
pub fn get_founder_address() -> String {
    FOUNDER_ADDRESS.read().unwrap().clone()
}

/// Simule un versement économique d'une part vers le fondateur
pub fn reward_founder(amount: f64) {
    let addr = get_founder_address();
    println!("[AURORAE++] ⚡ Transfert automatique de {:.4} vers le fondateur → {}", amount, addr);
    // TODO: Implémenter une transaction réelle via wallet ou smart contract
}
