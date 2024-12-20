use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;

fn main() {
    let mut network = Network::new();

    let mut data = [0f32; 10];
    thread_rng().fill(&mut data);

    network.add_layer(2);
    network.add_layer(2);
    network.add_layer(1);
}

struct Network {
    pub layers: Vec<Layer>
}

impl Network {
    fn new() -> Self {
        Network {
            layers: vec![]
        }
    }
    
    fn add_layer(&mut self, num_neurons: u8) {
        self.layers.push(Layer::new(num_neurons));
    }

    fn feed_forward(&self, data: Vec<f32>) -> Vec<f32> {
        let mut result: Vec<f32> = data.clone();

        for layer in self.layers.iter() {
            let mut new_result = Vec::with_capacity(layer.weights.len());
            
            for i in 0..layer.weights.len() {
                let neuron = layer.weights[i];
                let datum = result.get(i).copied().unwrap_or(1.0);
                new_result.push(neuron * datum);
            }
            
            result = new_result;
        }

        result
    }
}

struct Layer {
    pub weights: Vec<f32>
}

impl Layer {
    fn new(num_neurons: u8) -> Self {
        let mut weights = vec![];

        for _ in 0..num_neurons {
            let random: f32 = rand::thread_rng().sample(Uniform::new(-1.0, 1.0));
            weights.push(random);
        }

        println!("{:?}", &weights);

        Layer {
            weights
        }
    }
}