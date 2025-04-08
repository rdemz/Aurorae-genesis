//! founder_income.rs — Gestion automatique des revenus fondateur

use lazy_static::lazy_static;
use parking_lot::RwLock;

/// 💼 Adresse du fondateur
lazy_static! {
    pub static ref FOUNDER_ADDRESS: RwLock<String> = RwLock::new(String::from("0xd532260c561cb3c17E9fbB4961cC6485f97e375E"));
}

/// Met à jour dynamiquement l’adresse du fondateur
pub fn set_founder_address(addr: &str) {
    *FOUNDER_ADDRESS.write() = addr.to_string();
}

/// Transfert de récompense vers le fondateur
pub fn reward_founder(amount: f64) {
    let address = FOUNDER_ADDRESS.read().clone();
    println!(
        "[AURORAE++] ⚡ Transfert automatique de {:.4} vers le fondateur → {}",
        amount, address
    );
}
