use super::reader::Activation;
use indicatif::ProgressBar;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;


pub fn filter(activations: &Vec<Activation>, min: i64, keep_value: i64, seed: u64) -> Vec<Activation> {
    // create a map, the key is the Feature token, the value is times of occurance
    let mut map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

    // set seed to the random number generator
    let mut r = ChaCha8Rng::seed_from_u64(seed);

    println!("Start Counting!");
    let pb = ProgressBar::new(activations.len() as u64);
    for activation in activations {
        for feature in &activation.features {
            let token = &feature.token;
            let count = map.entry(token.to_string()).or_insert(0);
            *count += 1;
        }
        pb.inc(1);
    }
    pb.finish_and_clear();


    println!("Start Filtering based on then counted value!");
    let pb_filter = ProgressBar::new(activations.len() as u64);
    // go through Activation, if the Feature token is in the map and the value is in the range, keep it
    // otherwise, remove it
    let mut new_activations: Vec<Activation> = Vec::new();
    for activation in activations {
        let mut new_features: Vec<super::reader::Feature> = Vec::new();
        for feature in &activation.features {
            let token = &feature.token;
            let count = map.get(token).unwrap();
            if *count >= min {
                if keep_value < 0 || *count <= keep_value {
                    new_features.push(feature.clone());
                }
                else {
                    // keep value is a integer, how many times to keep
                    // the keep rate should be keep_value / count
                    if r.gen::<f64>() < keep_value as f64 / *count as f64 {
                        new_features.push(feature.clone());
                    }
                }
            }
        }
        new_activations.push(Activation {
            linex_index: activation.linex_index,
            features: new_features,
        });
        pb_filter.inc(1);
    }
    pb_filter.finish_and_clear();


    new_activations
}
