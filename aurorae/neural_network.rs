extern crate tch;
use tch::{nn, Device, Tensor};

// Crée un réseau de neurones avec des couches linéaires
pub struct DecisionNet {
    pub net: nn::Sequential,
}

impl DecisionNet {
    pub fn new(vs: &nn::VarStore, input_size: i64, hidden_sizes: Vec<i64>, output_size: i64) -> DecisionNet {
        let mut net = nn::seq();

        // Ajouter les couches cachées
        let mut prev_size = input_size;
        for &size in &hidden_sizes {
            net = net.add(nn::linear(vs, prev_size, size, Default::default()));
            net = net.add_fn(|xs| xs.relu());
            prev_size = size;
        }

        // Ajouter la couche de sortie
        net = net.add(nn::linear(vs, prev_size, output_size, Default::default()));

        DecisionNet { net }
    }

    pub fn forward(&self, input: Tensor) -> Tensor {
        self.net.forward(&input)
    }

    // Entraîner le réseau
    pub fn train(&self, input: Tensor, target: Tensor, optimizer: &mut nn::Optimizer<nn::Adam>) {
        // Faire une passe avant pour calculer la sortie
        let output = self.forward(input);

        // Calcul de la perte (MSE - Mean Squared Error)
        let loss = output.mse_loss(&target, tch::Reduction::Mean);

        // Faire une passe arrière pour calculer les gradients
        loss.backward();

        // Réinitialiser les gradients avant la mise à jour
        optimizer.zero_grad();

        // Appliquer les gradients
        optimizer.step();
    }
}

// Créer un optimiseur Adam
pub fn create_optimizer(vs: &nn::VarStore) -> nn::Optimizer<nn::Adam> {
    nn::Adam::default().build(vs, 1e-3).unwrap()  // Créer l'optimiseur Adam avec un taux d'apprentissage de 1e-3
}
