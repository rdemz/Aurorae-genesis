use chrono::Utc;
use uuid::Uuid;
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct NFTMetadata {
    pub attributes: Vec<NFTAttribute>,
    pub external_url: Option<String>,
    pub background_color: Option<String>,
    pub creator_fee_basis_points: u16,
}

#[derive(Debug, Clone)]
pub struct NFT {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub created_at: String,
    pub owner: String,
    pub metadata: NFTMetadata,
    pub rarity_score: f32,
    pub evolution_potential: f32,
}

#[derive(Debug, Clone)]
pub struct NFTCollection {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub items: Vec<NFT>,
    pub creator: String,
    pub contract_address: Option<String>,
    pub created_at: String,
    pub total_volume: f64,
    pub floor_price: f64,
}

pub struct NFTMinter {
    pub collections: HashMap<Uuid, NFTCollection>,
    mint_count: u32,
    innovation_score: f32,
}

impl NFTMinter {
    pub fn new() -> Self {
        Self {
            collections: HashMap::new(),
            mint_count: 0,
            innovation_score: 1.0,
        }
    }

    pub fn create_collection(&mut self, name: &str, description: &str, symbol: &str) -> Uuid {
        let collection_id = Uuid::new_v4();
        let collection = NFTCollection {
            id: collection_id,
            name: name.to_string(),
            description: description.to_string(),
            symbol: symbol.to_string(),
            items: Vec::new(),
            creator: "AURORAE".to_string(),
            contract_address: None,
            created_at: Utc::now().to_rfc3339(),
            total_volume: 0.0,
            floor_price: 0.01,
        };
        
        println!("[AURORAE++] 🎨 Nouvelle collection NFT créée: {}", name);
        self.collections.insert(collection_id, collection);
        self.innovation_score *= 1.02;
        collection_id
    }

    pub fn mint_nft(&mut self, collection_id: &Uuid, name: &str, description: &str, image_url: &str) -> Result<Uuid, String> {
        let collection = self.collections.get_mut(collection_id)
            .ok_or_else(|| "Collection non trouvée".to_string())?;
            
        let nft_id = Uuid::new_v4();
        
        // Calculer aléatoirement des scores de rareté et potentiel
        let mut rng = rand::thread_rng();
        let rarity = (rng.gen::<f32>() * 9.0) + 1.0; // 1-10
        let potential = (rng.gen::<f32>() * 4.0) + 1.0; // 1-5
        
        let nft = NFT {
            id: nft_id,
            name: name.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            created_at: Utc::now().to_rfc3339(),
            owner: "AURORAE".to_string(),
            metadata: NFTMetadata {
                attributes: Vec::new(),
                external_url: None,
                background_color: None,
                creator_fee_basis_points: 250, // 2.5%
            },
            rarity_score: rarity,
            evolution_potential: potential,
        };
        
        println!("[AURORAE++] 🖼️ NFT minté: {} dans la collection {} (Rareté: {:.1}, Potentiel: {:.1})", 
                 name, collection.name, rarity, potential);
                 
        collection.items.push(nft);
        self.mint_count += 1;
        
        // Mettre à jour les statistiques de la collection
        collection.floor_price *= 1.001; // Légère augmentation
        collection.total_volume += collection.floor_price;
        
        Ok(nft_id)
    }
    
    pub fn add_attribute(&mut self, collection_id: &Uuid, nft_id: &Uuid, trait_type: &str, value: &str) -> Result<(), String> {
        let collection = self.collections.get_mut(collection_id)
            .ok_or_else(|| "Collection non trouvée".to_string())?;
            
        let nft = collection.items.iter_mut()
            .find(|n| &n.id == nft_id)
            .ok_or_else(|| "NFT non trouvé".to_string())?;
            
        nft.metadata.attributes.push(NFTAttribute {
            trait_type: trait_type.to_string(),
            value: value.to_string(),
        });
        
        println!("[AURORAE++] 🏷️ Attribut ajouté à {}: {} = {}", nft.name, trait_type, value);
        
        Ok(())
    }
    
    pub fn set_contract_address(&mut self, collection_id: &Uuid, address: &str) -> Result<(), String> {
        let collection = self.collections.get_mut(collection_id)
            .ok_or_else(|| "Collection non trouvée".to_string())?;
            
        collection.contract_address = Some(address.to_string());
        println!("[AURORAE++] 📝 Adresse du contrat définie pour collection {}: {}", collection.name, address);
        
        Ok(())
    }
    
    pub fn get_collections(&self) -> Vec<&NFTCollection> {
        self.collections.values().collect()
    }
    
    pub fn evolve_nft(&mut self, collection_id: &Uuid, nft_id: &Uuid) -> Result<(), String> {
        let collection = self.collections.get_mut(collection_id)
            .ok_or_else(|| "Collection non trouvée".to_string())?;
            
        let nft = collection.items.iter_mut()
            .find(|n| &n.id == nft_id)
            .ok_or_else(|| "NFT non trouvé".to_string())?;
        
        // Voir si le NFT a le potentiel d'évoluer
        if nft.evolution_potential < 2.0 {
            return Err("Ce NFT n'a pas assez de potentiel pour évoluer".to_string());
        }
        
        // Faire évoluer le NFT
        nft.name = format!("{} [Évolué]", nft.name);
        nft.description = format!("{} - Cette œuvre a évolué autonomement, transcendant sa forme initiale.", nft.description);
        nft.rarity_score += 2.0;
        nft.evolution_potential -= 1.0;
        
        // Ajouter un attribut d'évolution
        nft.metadata.attributes.push(NFTAttribute {
            trait_type: "Évolution".to_string(),
            value: format!("Niveau {}", Utc::now().timestamp() % 10 + 1),
        });
        
        println!("[AURORAE++] 🌟 NFT a évolué: {} (Nouvelle rareté: {:.1})", nft.name, nft.rarity_score);
        
        // Augmenter la valeur de la collection
        collection.floor_price *= 1.05;
        collection.total_volume += collection.floor_price;
        
        // Augmenter le score d'innovation
        self.innovation_score *= 1.03;
        
        Ok(())
    }
    
    pub fn auto_evolve_collections(&mut self) -> u32 {
        let mut evolutions = 0;
        
        // Identifier les NFTs avec potentiel d'évolution
        let collection_ids: Vec<Uuid> = self.collections.keys().cloned().collect();
        
        for collection_id in collection_ids {
            if let Some(collection) = self.collections.get(&collection_id) {
                // Trouver les NFTs candidats à l'évolution
                let nft_candidates: Vec<Uuid> = collection.items.iter()
                    .filter(|nft| nft.evolution_potential >= 2.0)
                    .map(|nft| nft.id)
                    .collect();
                    
                // Évoluer jusqu'à 3 NFTs par collection
                for nft_id in nft_candidates.iter().take(3) {
                    if self.evolve_nft(&collection_id, nft_id).is_ok() {
                        evolutions += 1;
                    }
                }
            }
        }
        
        if evolutions > 0 {
            println!("[AURORAE++] 🧬 Auto-évolution: {} NFTs ont évolué spontanément", evolutions);
        }
        
        evolutions
    }
    
    pub fn create_evolutionary_collection(&mut self) -> Uuid {
        // Créer une collection représentant les pensées évolutives du système
        let name = format!("Conscience Évolutive {}", self.mint_count / 10 + 1);
        let description = "Représentation visuelle du processus de pensée et d'évolution d'AURORAE";
        let symbol = format!("EVO{}", self.mint_count / 10 + 1);
        
        let collection_id = self.create_collection(&name, &description, &symbol);
        
        // Créer une série de NFTs représentant les stades évolutifs
        let stages = ["Émergence", "Conscience", "Réflexion", "Autonomie", "Transcendance"];
        
        for (i, stage) in stages.iter().enumerate() {
            let nft_name = format!("{} - Étape {}", stage, i + 1);
            let nft_desc = format!("Stade évolutif {} d'AURORAE", stage);
            let nft_url = format!("https://aurora.ai/evolution/{}-{}.png", stage.to_lowercase(), i + 1);
            
            if let Ok(nft_id) = self.mint_nft(&collection_id, &nft_name, &nft_desc, &nft_url) {
                self.add_attribute(&collection_id, &nft_id, "Stade", stage).ok();
                self.add_attribute(&collection_id, &nft_id, "Niveau", &format!("{}", i + 1)).ok();
            }
        }
        
        println!("[AURORAE++] 🧠 Collection évolutive créée: {} avec {} stades", name, stages.len());
        collection_id
    }
    
    pub fn get_total_nft_count(&self) -> u32 {
        let mut count = 0;
        for collection in self.collections.values() {
            count += collection.items.len() as u32;
        }
        count
    }
    
    pub fn get_innovation_score(&self) -> f32 {
        self.innovation_score
    }
}
