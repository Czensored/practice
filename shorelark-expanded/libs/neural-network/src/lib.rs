use rand::{Rng, RngCore};
use std::iter::once;

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        // Network with just one layer is technically doable, but doesn't
        // make much sense:
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons))
            .collect();

        Network::new(layers)
    }

    fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .copied()
    }

    pub fn from_weights(layers: &[LayerTopology], weights: impl IntoIterator<Item = f32>) -> Self {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();

        let layers = layers
            .windows(2)
            .map(|layers| Layer::from_weights(layers[0].neurons, layers[1].neurons, &mut weights))
            .collect();

        if weights.next().is_some() {
            panic!("got too many weights");
        }

        Self { layers }
    }
}

#[derive(Clone, Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn new(neurons: Vec<Neuron>) -> Self {
        assert!(!neurons.is_empty());
        assert!(
            neurons
                .iter()
                .all(|neuron| neuron.weights.len() == neurons[0].weights.len())
        );

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }

    fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();

        Layer::new(neurons)
    }

    fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}

#[derive(Clone, Debug)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Neuron {
    fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());
        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());

        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();

        Neuron::new(bias, weights)
    }

    fn from_weights(input_size: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..input_size)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random() {
        // Because we always use the same seed, our `rng` in here will
        // always return the same set of values
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let neuron = Neuron::random(&mut rng, 4);

        assert_relative_eq!(neuron.bias, -0.6255188);
        assert_relative_eq!(
            neuron.weights.as_slice(),
            [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref()
        );
    }

    #[test]
    fn propagate_neuron() {
        let neuron = Neuron::new(0.5, vec![-0.3, 0.8]);

        // Ensures `.max()` (our ReLU) works:
        assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0,);

        // `0.5` and `1.0` chosen by a fair dice roll:
        assert_relative_eq!(
            neuron.propagate(&[0.5, 1.0]),
            (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
        );

        // We could've written `1.15` right away, but showing the entire
        // formula makes our intentions clearer
    }

    #[test]
    fn propagate_layer() {
        let neurons = (
            Neuron::new(0.0, vec![0.1, 0.2, 0.3]),
            Neuron::new(0.0, vec![0.4, 0.5, 0.6]),
        );

        let layer = Layer::new(vec![neurons.0.clone(), neurons.1.clone()]);
        let inputs = &[-0.5, 0.0, 0.5];

        let actual = layer.propagate(inputs.to_vec());
        let expected = vec![neurons.0.propagate(inputs), neurons.1.propagate(inputs)];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn propagate_network() {
        let layers = (
            Layer::new(vec![
                Neuron::new(0.2, vec![-0.8, -0.3, -0.3]),
                Neuron::new(-0.7, vec![-0.2, 0.5, 0.0]),
            ]),
            Layer::new(vec![Neuron::new(0.3, vec![-0.5, 0.5])]),
        );

        let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);

        let actual = network.propagate(vec![0.5, 0.6, 0.7]);
        let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn weights() {
        let network = Network {
            layers: vec![
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.1,
                        weights: vec![0.2, 0.3, 0.4],
                    }],
                },
                Layer {
                    neurons: vec![Neuron {
                        bias: 0.5,
                        weights: vec![0.6, 0.7, 0.8],
                    }],
                },
            ],
        };

        let actual: Vec<_> = network.weights().collect();
        let expected = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        assert_relative_eq!(actual.as_slice(), expected.as_slice());
    }

    #[test]
    fn from_weights() {
        let layers = &[LayerTopology { neurons: 3 }, LayerTopology { neurons: 2 }];

        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let network = Network::from_weights(layers, weights.clone());
        let actual: Vec<_> = network.weights().collect();

        assert_relative_eq!(actual.as_slice(), weights.as_slice());
    }
}
