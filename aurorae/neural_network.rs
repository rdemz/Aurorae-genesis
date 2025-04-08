extern crate tch;
use tch::{Tensor, Device, nn, nn::Module, nn::OptimizerConfig, no_grad};

#[derive(Debug)]
pub struct DecisionNet {
    pub net: nn::Sequential,
}

impl DecisionNet {
    // Créez le réseau de neurones avec plusieurs couches
    pub fn new(vs: &nn::VarStore, input_size: i64, hidden_sizes: Vec<i64>, output_size: i64) -> DecisionNet {
        let mut net = nn::seq();

        // Ajouter les couches cachées
        let mut prev_size = input_size;
        for &size in &hidden_sizes {
            net = net.add(nn::linear(vs.root(), prev_size, size, Default::default()));  // Utilisation de vs.root()
            net = net.add_fn(|xs| xs.relu());
            prev_size = size;
        }

        // Ajouter la couche de sortie
        net = net.add(nn::linear(vs.root(), prev_size, output_size, Default::default()));  // Utilisation de vs.root()

        DecisionNet { net }
    }

    // Passer les entrées à travers le réseau pour obtenir la prédiction
    pub fn forward(&self, input: Tensor) -> Tensor {
        self.net.forward(&input)
    }

    // Entraîner le réseau de neurones
    pub fn train(&self, input: Tensor, target: Tensor, optimizer: &mut nn::Adam) {
        // Forward pass : calculer la sortie
        let output = self.forward(input);

        // Calcul de la perte (MSE - Mean Squared Error)
        let loss = output.mse_loss(&target, tch::Reduction::Mean);

        // Backward pass : calculer les gradients
        loss.backward();

        // Mettre à jour les poids du réseau
        optimizer.zero_grad();  // Réinitialiser les gradients avant la mise à jour
        optimizer.step();       // Appliquer les gradients
    }
}

pub fn create_optimizer(vs: &nn::VarStore) -> nn::Optimizer<nn::Adam> {
    nn::Adam::default().build(vs, 1e-3).unwrap()  // Crée l'optimiseur Adam avec un taux d'apprentissage de 1e-3
}
