extern crate rand;
extern crate ndarray;


use ndarray::{Array2};

/// create a layer with random numbers
fn build_layer(nodes: u32) -> [f64] {
    let mut rng = rand::thread_rng();
    let mut layer = vec![0, nodes];
    for i in 0..layer.len() {
        layer[i] = rng.gen::<f64>();
    }
    return &layer;
}

fn build_network(input_layer: u32, hidden_layer: u32, output_layer: u32) -> [[f64; 2]; 3] {
    let mut network = Vec::new();
    network.push(build_layer(input_layer));
    network.push(build_layer(hidden_layer));
    network.push(build_layer(output_layer));
    return &network;
}

fn main() {
    let network = build_network(2, 2, 2);
    for layer in network.iter() {
        for weight in layer.iter() {
            println!("{}", weight);
        }
    }
}
