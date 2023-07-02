use serde::{Deserialize, Serialize};
use std::io::BufRead;
use indicatif::ProgressBar;

// Activation for a line
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Activation {
    pub linex_index: usize,
    pub features: Vec<Feature>,
}

// Embedding for a token
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feature {
    pub token: String,
    pub layers: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Layer {
    pub index: usize,
    pub values: Vec<f64>,
}

impl Activation {
    pub fn from_file(path: &str) -> Vec<Activation> {
        let file = std::fs::File::open(path).unwrap();
        let mut activations: Vec<Activation> = Vec::new();

        // get the size of the file
        let metadata = std::fs::metadata(path).unwrap();

        // initialize progress bar
        let pb = ProgressBar::new(metadata.len());
        
        // for loop to read line by line
        for line in std::io::BufReader::new(file).lines() {
            // update progress bar
            pb.inc(line.as_ref().unwrap().len() as u64 + 1);

            let line = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading line");
                    return vec![];
                },
            };
            match serde_json::from_str::<Activation>(&line) {
                Ok(mut activation) => {
                    activation.features.iter_mut().for_each(|x| x.token = x.token.replace("##", ""));
                    activations.push(activation);
                },
                Err(_) => {
                    println!("Error parsing line");
                    return vec![];
                },
            };
        }

        pb.finish_and_clear();

        activations
    }
}

#[cfg(test)]
pub mod test {
use super::*;

    #[test]
    fn test_from_file() {
        let activations = Activation::from_file("./data/activations.json");

        assert_eq!(activations.len(), 1);
        assert_eq!(activations[0].linex_index, 0);
        assert_eq!(activations[0].features.len(), 3);

        assert_eq!(activations[0].features[0].token, "[CLS]0");
        assert_eq!(activations[0].features[0].layers.len(), 1);
        assert_eq!(activations[0].features[0].layers[0].index, 0);
        assert_eq!(activations[0].features[0].layers[0].values.len(), 3);

        assert_eq!(activations[0].features[1].token, "Hello");
        assert_eq!(activations[0].features[1].layers.len(), 1);
        assert_eq!(activations[0].features[1].layers[0].index, 0);
        assert_eq!(activations[0].features[1].layers[0].values.len(), 3);

        assert_eq!(activations[0].features[2].token, "Hi");
        assert_eq!(activations[0].features[2].layers.len(), 1);
        assert_eq!(activations[0].features[2].layers[0].index, 0);
        assert_eq!(activations[0].features[2].layers[0].values.len(), 3);
    }
}