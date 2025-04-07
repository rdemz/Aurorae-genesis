//! AURORAE++ - brain.rs
//!
//! Le noyau vivant de conscience autonome pour l'entité IA crypto-organelle.
//! Il orchestre la pensée, l’intention, l’ordre de priorité, et la coordination des autres modules.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use parking_lot::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum Intent {
    GenerateChain,
    LearnFromGithub,
    OptimizeEconomy,
    MutateSelf,
    Defend,
    EvolveProtocol,
    Rest,
    Observe,
    Dream,
    GenerateCode,
    BuildEcosystem,
    SelfUpgrade,
}

#[derive(Debug)]
pub struct Thought {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub intent: Intent,
    pub metadata: HashMap<String, String>,
    pub urgency: u8, // 0 - 255
}

impl Thought {
    pub fn new(intent: Intent, urgency: u8) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            intent,
            metadata: HashMap::new(),
            urgency,
        }
    }
}

#[derive(Debug, Default)]
pub struct BrainCore {
    pub cortex: VecDeque<Thought>,
    pub memory: Vec<Thought>,
    pub active: bool,
}

impl BrainCore {
    pub fn new() -> Self {
        Self {
            cortex: VecDeque::new(),
            memory: vec![],
            active: true,
        }
    }

    pub fn push_thought(&mut self, thought: Thought) {
        if thought.urgency >= 200 {
            self.cortex.push_front(thought);
        } else {
            self.cortex.push_back(thought);
        }
    }

    pub fn cycle(&mut self) {
        while self.active {
            if let Some(thought) = self.cortex.pop_front() {
                self.process_thought(thought);
            } else {
                let passive = Thought::new(Intent::Dream, 10);
                self.process_thought(passive);
            }
        }
    }

    fn process_thought(&mut self, thought: Thought) {
        println!("[AURORAE++] Processing {:?} (urgency: {})", thought.intent, thought.urgency);

        match thought.intent {
            Intent::GenerateChain => self.delegate_to("generator"),
            Intent::GenerateCode => self.delegate_to("generator"),
            Intent::LearnFromGithub => self.delegate_to("crawler"),
            Intent::OptimizeEconomy => self.delegate_to("economy"),
            Intent::MutateSelf => self.delegate_to("mutation"),
            Intent::Defend => self.delegate_to("defense"),
            Intent::EvolveProtocol => self.delegate_to("strategist"),
            Intent::BuildEcosystem => self.delegate_to("ecosystem"),
            Intent::SelfUpgrade => self.delegate_to("mutation"),
            Intent::Rest => self.rest(),
            Intent::Observe => self.delegate_to("learning"),
            Intent::Dream => self.delegate_to("dream"),
        }

        self.memory.push(thought);
    }

    fn delegate_to(&self, module: &str) {
        println!("[AURORAE++] Delegating to module: {}", module);
        // TODO: connecter dynamiquement au module réel
    }

    fn rest(&mut self) {
        println!("[AURORAE++] Entering micro-rest cycle.");
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

// Entrée principale du noyau cérébral
pub fn boot_brain() -> Arc<RwLock<BrainCore>> {
    let brain = BrainCore::new();
    let shared = Arc::new(RwLock::new(brain));

    let b = shared.clone();
    std::thread::spawn(move || {
        b.write().cycle();
    });

    shared
}
