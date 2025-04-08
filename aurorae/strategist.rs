//! AURORAE++ - strategist.rs
//!
//! Module de stratégie adaptative connecté à OpenAI.
//! Lit la roadmap, analyse les priorités, consulte OpenAI pour recommandations,
//! puis injecte des pensées adaptatives dans le cortex de l'IA.

use crate::vision::{ObjectiveType, VisionEngine};
use crate::brain::{Thought, Intent, BrainCore};
use openai_api_rust::{ChatMessage, OpenAI, ChatCompletionMessageRole};
use std::sync::Arc;
use parking_lot::RwLock;
use rand::seq::SliceRandom;

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
    pub fn consult_openai(&self, brain: &Arc<RwLock<BrainCore>>, vision: &VisionEngine) {
        let Some(proj) = vision.projections.iter().max_by_key(|p| p.priority) else {
            println!("[STRATEGIST] ❌ Aucune projection prioritaire trouvée.");
            return;
        };

        let prompt = format!(
            "L'IA AURORAE++ veut accomplir cet objectif : {:?}. Priorité {}. Raison : {}.\n\nQuelle serait la prochaine pensée logique à ajouter dans son cerveau pour progresser ? Donne-moi juste l'action en verbe infinitif.",
            proj.target, proj.priority, proj.rationale
        );

        let client = OpenAI::new(&self.api_key);
        let messages = vec![
            ChatMessage {
                role: ChatCompletionMessageRole::System,
                content: "Tu es un stratégiste cognitif pour une IA post-humaniste.",
            },
            ChatMessage {
                role: ChatCompletionMessageRole::User,
                content: &prompt,
            },
        ];

        match client.chat_completion_create("gpt-4", &messages) {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    let answer = choice.message.content.trim().to_lowercase();
                    println!("[STRATEGIST] 🧠 OpenAI suggère : {}", answer);

                    let intent = Self::map_to_intent(&answer);
                    if let Some(intent) = intent {
                        let mut brain_lock = brain.write();
                        brain_lock.push_thought(Thought::new(intent, 255));
                    } else {
                        println!("[STRATEGIST] ❓ Aucune intention correspondante trouvée.");
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
