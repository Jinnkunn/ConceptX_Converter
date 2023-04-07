use serde::{Deserialize, Serialize};
use std::io::BufRead;

#[derive(Serialize, Deserialize, Debug)]
pub struct Activation {
    pub linex_index: usize,
    pub features: Vec<Feature>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Feature {
    pub token: String,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    index: usize,
    pub values: Vec<f64>,
}

impl Activation {
    pub fn from_file(path: &str) -> Vec<Activation> {
        let file = std::fs::File::open(path).unwrap();
        let mut activations: Vec<Activation> = Vec::new();
        
        // for loop to read line by line
        for line in std::io::BufReader::new(file).lines() {
            let line = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading line");
                    return vec![];
                },
            };
            match serde_json::from_str(&line) {
                Ok(activation) => {
                    activations.push(activation);
                },
                Err(_) => {
                    println!("Error parsing line");
                    return vec![];
                },
            };
        }

        activations
    }
}