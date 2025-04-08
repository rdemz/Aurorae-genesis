//! AURORAE++ - strategist.rs
//!
//! Module de stratégie adaptative connecté à OpenAI.
//! Lit la roadmap, analyse les priorités, consulte OpenAI pour recommandations,
//! puis injecte des pensées adaptatives dans le cortex de l'IA.

use crate::vision::{ObjectiveType, VisionEngine};
use crate::brain::{Thought, Intent, BrainCore};
use std::sync::Arc;
use parking_lot::RwLock;
use rand::Rng;
use async_openai::types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role};
use async_openai::Client;

pub struct Strategist {
    pub api_key: String,
}

impl Strategist {
    pub fn new(api_key: &str) -> Self {
        Self {
            api_key: api_key.to_string(),
        }
    }

    /// 🔮 Analyse la roadmap et injecte des pensées inspirées d'OpenAI
    pub async fn consult_openai(&self, brain: &Arc<RwLock<BrainCore>>, vision: &mut VisionEngine) {
        let Some(proj) = vision.projections.iter().max_by_key(|p| p.priority) else {
            println!("[STRATEGIST] ❌ Aucune projection prioritaire trouvée.");
            return;
        };

        let prompt = format!(
            "L'IA AURORAE++ veut accomplir cet objectif : {:?}. Priorité {}. Raison : {}.\n\nQuelle serait la prochaine pensée logique à ajouter dans son cerveau pour progresser ? Donne-moi juste l'action en verbe infinitif.",
            proj.target, proj.priority, proj.rationale
        );

        let client = Client::new();
        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4")
            .messages([ 
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content("Tu es un stratégiste cognitif pour une IA post-humaniste.")
                    .build()
                    .unwrap(),
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(&prompt)
                    .build()
                    .unwrap(),
            ])
            .max_tokens(60)
            .build()
            .unwrap();

        match client.chat().create(request).await {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    // Traite la réponse potentiellement Option<String>
                    if let Some(content) = choice.message.content.as_ref() {
                        let answer = content.trim().to_lowercase();
                        println!("[STRATEGIST] 🧠 OpenAI suggère : {}", answer);

                        let intent = Self::map_to_intent(&answer);
                        let urgency = 220 + rand::thread_rng().gen_range(0..=30);

                        let mut brain_lock = brain.write();
                        if let Some(intent) = intent {
                            brain_lock.push_thought(Thought::new(intent, urgency));
                        } else {
                            let fallback = Intent::Observe;
                            println!("[STRATEGIST] ❓ Aucune intention reconnue, fallback vers {:?}", fallback);
                            brain_lock.push_thought(Thought::new(fallback, 128));
                        }
                    } else {
                        println!("[STRATEGIST] ⚠️ Aucune réponse de contenu retournée.");
                    }
                }
            }
            Err(e) => println!("[STRATEGIST] ❌ Erreur OpenAI : {}", e),
        }
    }

    fn map_to_intent(answer: &str) -> Option<Intent> {
        let table = vec![
            ("apprendre", Intent::LearnFromGithub),
            ("muter", Intent::MutateSelf),
            ("optimiser", Intent::OptimizeEconomy),
            ("créer", Intent::GenerateCode),
            ("observer", Intent::Observe),
            ("reposer", Intent::Rest),
            ("reproduire", Intent::SelfReplicate),
            ("construire", Intent::BuildEcosystem),
            ("prodiguer", Intent::EvolveProtocol),
            ("défendre", Intent::Defend),
        ];

        for (keyword, intent) in table {
            if answer.contains(keyword) {
                return Some(intent);
            }
        }
        None
    }
}
