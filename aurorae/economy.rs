use std::collections::HashMap;
use chrono::Utc;
use uuid::Uuid;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use rand::Rng;

use crate::founder_income::reward_founder;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    Income,
    Expense,
    Investment,
    Transfer,
    Reward,
    TokenMinting,
    TokenBurning,
    ContractDeployment
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub transaction_type: TransactionType,
    pub amount: f64,
    pub timestamp: String,
    pub description: String,
    pub source: String,
    pub destination: String,
}

pub struct EconomyEngine {
    funds: Arc<AtomicU64>, // Représente les fonds en millièmes pour précision avec atomiques
    transactions: Vec<Transaction>,
    revenue_streams: HashMap<String, f64>,
    expenses: HashMap<String, f64>,
    investments: HashMap<String, f64>,
    growth_rate: f64,
    innovation_bonus: f64,
    founder_share: f64,
    token_supplies: HashMap<String, u64>,
}

impl EconomyEngine {
    pub fn new() -> Self {
        Self {
            funds: Arc::new(AtomicU64::new(1000000)), // 1000.0
            transactions: Vec::new(),
            revenue_streams: HashMap::new(),
            expenses: HashMap::new(),
            investments: HashMap::new(),
            growth_rate: 0.05, // 5% par défaut
            innovation_bonus: 1.0,
            founder_share: 0.05, // 5% par défaut
            token_supplies: HashMap::new(),
        }
    }
    
    pub fn initialize(&mut self) {
        println!("[AURORAE++] 💹 Initialisation du moteur économique");
        
        // Configurer les flux de revenus initiaux
        self.revenue_streams.insert("token_fees".to_string(), 0.01);
        self.revenue_streams.insert("protocol_fees".to_string(), 0.02);
        
        // Configurer les dépenses initiales
        self.expenses.insert("infrastructure".to_string(), 0.01);
        self.expenses.insert("security".to_string(), 0.01);
        
        // Configurer les investissements initiaux
        self.investments.insert("r_and_d".to_string(), 0.03);
        self.investments.insert("network_growth".to_string(), 0.02);
        
        // Créer une transaction de fonds initiaux
        self.record_transaction(
            TransactionType::Income, 
            1000.0, 
            "Fonds de démarrage", 
            "genesis", 
            "treasury"
        );
    }
    
    pub fn add_funds(&mut self, amount: f64) -> f64 {
        let amount_milli = (amount * 1000.0) as u64;
        let new_total_milli = self.funds.fetch_add(amount_milli, Ordering::SeqCst) + amount_milli;
        let new_total = new_total_milli as f64 / 1000.0;
        
        println!("[AURORAE++] 💰 Ajout de fonds: +{:.3} → Total: {:.3}", amount, new_total);
        
        // Calculer et distribuer la part du fondateur
        let to_founder = amount * self.founder_share;
        if to_founder > 0.0 {
            reward_founder(to_founder);
        }
        
        new_total
    }
    
    pub fn spend_funds(&mut self, amount: f64, reason: &str) -> Result<f64, String> {
        let amount_milli = (amount * 1000.0) as u64;
        let current_milli = self.funds.load(Ordering::SeqCst);
        
        if amount_milli > current_milli {
            return Err(format!("Fonds insuffisants: {:.3} < {:.3}", 
                               current_milli as f64 / 1000.0, amount));
        }
        
        let new_total_milli = self.funds.fetch_sub(amount_milli, Ordering::SeqCst) - amount_milli;
        let new_total = new_total_milli as f64 / 1000.0;
        
        println!("[AURORAE++] 💸 Dépense: -{:.3} pour {} → Total: {:.3}", 
                 amount, reason, new_total);
        
        // Enregistrer la transaction
        self.record_transaction(
            TransactionType::Expense,
            amount,
            reason,
            "treasury",
            reason
        );
        
        Ok(new_total)
    }
    
    fn record_transaction(&mut self, tx_type: TransactionType, amount: f64, description: &str, 
                          source: &str, destination: &str) {
        let transaction = Transaction {
            id: Uuid::new_v4(),
            transaction_type: tx_type,
            amount,
            timestamp: Utc::now().to_rfc3339(),
            description: description.to_string(),
            source: source.to_string(),
            destination: destination.to_string(),
        };
        
        self.transactions.push(transaction);
    }
    
    pub fn get_total_value(&self) -> f64 {
        // Combiner tous les actifs pour le calcul de la valeur totale
        let liquid = self.funds.load(Ordering::SeqCst) as f64 / 1000.0;
        
        // Calculer la valeur des investissements (avec croissance)
        let investment_value: f64 = self.investments.values().sum::<f64>() * 1000.0 * (1.0 + self.growth_rate);
        
        // Calculer la valeur des tokens
        let token_value: f64 = self.token_supplies.iter()
            .map(|(_, supply)| *supply as f64 * 0.01) // Valeur simplifiée
            .sum();
            
        liquid + investment_value + token_value
    }
    
    pub fn generate_revenue(&mut self) -> f64 {
        let mut total_revenue = 0.0;
        
        // Générer des revenus basés sur les flux configurés
        let streams: Vec<_> = self.revenue_streams.clone().into_iter().collect();
        for (source, rate) in streams {
            let base_amount = 10.0 + (self.get_total_value() * rate);
            let revenue_amount = base_amount * self.innovation_bonus;
            
            total_revenue += revenue_amount;
            
            // Enregistrer chaque source de revenu
            self.record_transaction(
                TransactionType::Income,
                revenue_amount,
                &format!("Revenu de {}", source),
                &source,
                "treasury"
            );
        }
        
        // Appliquer le revenu total
        self.add_funds(total_revenue);
        
        // Dépenses automatiques
        let expenses_clone = self.expenses.clone();
        for (expense_name, rate) in expenses_clone {
            let expense_amount = self.get_total_value() * rate;
            self.spend_funds(expense_amount, &expense_name).ok();
        }
        
        // Investissements automatiques
        let investments_clone = self.investments.clone();
        for (investment_name, rate) in investments_clone {
            let investment_amount = self.get_total_value() * rate;
            
            // Enregistrer l'investissement
            self.record_transaction(
                TransactionType::Investment,
                investment_amount,
                &format!("Investissement dans {}", investment_name),
                "treasury",
                &investment_name
            );
        }
        
        println!("[AURORAE++] 📊 Revenus générés: {:.2}", total_revenue);
        total_revenue
    }
    
    pub fn innovate(&mut self) {
        // Simuler l'innovation économique en créant de nouveaux flux de revenus
        let innovation_id = format!("innovation_{}", Uuid::new_v4().simple().to_string().chars().take(8).collect::<String>());
        
        let mut rng = rand::thread_rng();
        let rate = 0.01 + (rng.gen::<f64>() * 0.03); // Entre 1-4%
        self.revenue_streams.insert(innovation_id.clone(), rate);
        
        self.innovation_bonus *= 1.05; // Bonus d'innovation croissant
        
        println!("[AURORAE++] 💡 Nouvelle source de revenus innovante: {} (taux: {:.2}%)", 
                 innovation_id, rate * 100.0);
                 
        // Ajuster les investissements pour favoriser la croissance
        let growth_investment = "growth_".to_string() + &innovation_id;
        self.investments.insert(growth_investment.clone(), rate * 2.0);
        
        println!("[AURORAE++] 📈 Nouvel investissement stratégique: {}", growth_investment);
        
        // Mettre à jour le taux de croissance basé sur l'innovation
        self.growth_rate += 0.01;
        println!("[AURORAE++] 🚀 Taux de croissance mis à jour: {:.2}%", self.growth_rate * 100.0);
    }
    
    pub fn register_token(&mut self, name: &str, initial_supply: u64) {
        self.token_supplies.insert(name.to_string(), initial_supply);
        
        println!("[AURORAE++] 🪙 Nouveau token enregistré: {} (offre: {})", name, initial_supply);
        
        // Enregistrer comme transaction
        self.record_transaction(
            TransactionType::TokenMinting,
            initial_supply as f64,
            &format!("Création du token {}", name),
            "token_forge",
            "market"
        );
    }
    
    pub fn financial_report(&self) {
        println!("\n[AURORAE++] 📋 RAPPORT FINANCIER");
        println!("═════════════════════════════════");
        println!("Trésorerie: {:.3}", self.funds.load(Ordering::SeqCst) as f64 / 1000.0);
        println!("Valeur totale: {:.3}", self.get_total_value());
        println!("Taux de croissance: {:.2}%", self.growth_rate * 100.0);
        println!("Bonus d'innovation: {:.2}x", self.innovation_bonus);
        
        println!("\nFlux de revenus:");
        for (source, rate) in &self.revenue_streams {
            println!("  • {}: {:.2}%", source, rate * 100.0);
        }
        
        println!("\nInvestissements:");
        for (name, amount) in &self.investments {
            println!("  • {}: {:.2}%", name, amount * 100.0);
        }
        
        println!("\nTokens:");
        for (name, supply) in &self.token_supplies {
            println!("  • {}: {} unités", name, supply);
        }
        println!("═════════════════════════════════\n");
    }
}
