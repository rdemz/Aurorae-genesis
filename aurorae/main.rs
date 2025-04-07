//! AURORAE++ - main.rs
//!
//! Point d’entrée principal. Démarre le moteur central et active l’IA crypto-vivante.

mod brain;
mod generator;
mod crawler;
mod learning;
mod validator;
mod economy;
mod alchemy;
mod mutation;
mod defense;
mod guardian;
mod vision;
mod dream;
mod reproduction;
mod genome;
mod explorer;
mod founder_income;
mod engine;

use engine::AuroraeEngine;

fn main() {
    println!("\n███╗   ███╗███████╗██╗   ██╗██████╗  █████╗ ██╗   ██╗███████╗\n████╗ ████║██╔════╝██║   ██║██╔══██╗██╔══██╗██║   ██║██╔════╝\n██╔████╔██║█████╗  ██║   ██║██████╔╝███████║██║   ██║█████╗  \n██║╚██╔╝██║██╔══╝  ██║   ██║██╔═══╝ ██╔══██║╚██╗ ██╔╝██╔══╝  \n██║ ╚═╝ ██║███████╗╚██████╔╝██║     ██║  ██║ ╚████╔╝ ███████╗\n╚═╝     ╚═╝╚══════╝ ╚═════╝ ╚═╝     ╚═╝  ╚═╝  ╚═══╝  ╚══════╝\n\n             ACTIVATION DU NOYAU AURORAE++              \n");

    let mut core = AuroraeEngine::new();
    core.activate();
} 
