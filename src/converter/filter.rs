use super::reader::Activation;
use indicatif::ProgressBar;


pub fn filter(activations: &Vec<Activation>, min: i64, max: i64, keep_value: f64) -> Vec<Activation> {
    // create a map, the key is the Feature token, the value is times of occurance
    let mut map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

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


    println!("Start Filtering based on then counted value!");
    let pb = ProgressBar::new(activations.len() as u64);
    // go through Activation, if the Feature token is in the map and the value is in the range, keep it
    // otherwise, remove it
    let mut new_activations: Vec<Activation> = Vec::new();
    for activation in activations {
        let mut new_features: Vec<super::reader::Feature> = Vec::new();
        for feature in &activation.features {
            let token = &feature.token;
            let count = map.get(token).unwrap();
            if *count >= min && *count <= max {
                if keep_value < 0.0 {
                    new_features.push(feature.clone());
                }
                else {
                    if rand::random::<f64>() < keep_value / *count as f64 {
                        new_features.push(feature.clone());
                    }
                }
            }
        }
        new_activations.push(Activation {
            linex_index: activation.linex_index,
            features: new_features,
        });
        pb.inc(1);
    }


    new_activations
}
