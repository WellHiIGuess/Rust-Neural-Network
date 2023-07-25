use crate::node::Node;

pub struct NeuralNetwork {
    network: Vec<Vec<Node>>,
}

impl NeuralNetwork {
    // length: amount of nodes per layer
    // depth: amount of layers
    pub fn new(length: usize, depth: usize) -> Self {
        Self {
            network: vec![vec![Node::new(0.0); length]; depth]
        }
    }

    pub fn train() {
        // training algorithm
    }
}
