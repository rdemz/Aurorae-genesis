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
        let rarity = (rand::Rng::gen_range(&mut rng, 0.0..9.0)) + 1.0; // 1-10
        let potential = (rand::Rng::gen_range(&mut rng, 0.0..4.0)) + 1.0; // 1-5
        
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
    
    pub fn get_collections(&self) ->
