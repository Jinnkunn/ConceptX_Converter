use std::io::Write;
use indicatif::ProgressBar;

use super::reader::Activation;

pub fn to_w2v(activations: &Vec<Activation>, output_file_name: &str) {
    // check if output_file_name exists
    // if exists, delete it
    // create output_file_name
    if std::path::Path::new(output_file_name).exists() {
        std::fs::remove_file(output_file_name).unwrap();
    }
    let mut output_file = std::fs::File::create(output_file_name).unwrap();

    // write number of activations and vector size to the first line
    let number_of_tokens = activations.iter().map(|x| x.features.len()).sum::<usize>();

    // find the first activation that has a feature
    let mut vec_size = 0;
    for activation in activations {
        if activation.features.len() > 0 {
            vec_size = activation.features[0].layers[0].values.len();
            break;
        }
    }

    // initialize progress bar
    let pb = ProgressBar::new(activations.len() as u64);

    writeln!(output_file, "{} {}", number_of_tokens, vec_size).unwrap();

    // for each activation
    for activation in activations {
        let mut features_index = 0;
        for feature in &activation.features {
            // the word will be in the format of "word:line_number:word_index"
            let word = format!("{}:{}:{}", feature.token, activation.linex_index, features_index);
            let vec = feature.layers[0].values.clone(); // only one layer
            let vec_str = vec.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
            // write word and vec to output_file
            writeln!(output_file, "{} {}", word, vec_str).unwrap();
            features_index += 1;
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    // close output_file
    output_file.flush().unwrap();
}

