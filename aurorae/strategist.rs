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
use rand::Rng;

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
                if response.choices.is_empty() {
                    println!("[STRATEGIST] ⚠️ OpenAI n'a retourné aucune suggestion.");
                    return;
                }

                if let Some(choice) = response.choices.first() {
                    let answer = choice.message.content.trim().to_lowercase();
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
