use pyo3::prelude::*;

mod converter;

#[pyfunction]
fn processor(input: &str, output: &str, min: Option<i64>, max: Option<i64>, keep: Option<i64>) {
    // read read args
    // let args: Vec<String> = std::env::args().collect();
    // let input_file_name = &args[1];
    // let output_file_name = &args[2];
    println!("Start Reading! (This may take a while)");
    let mut activations = converter::reader::Activation::from_file(&input);

    let mut keep_value = -1;
    match keep {
        Some(keep) => {
            keep_value = keep;
        },
        _ => {},
    }

    // if min or max is None, then do not filter
    match (min, max) {
        (Some(min), Some(max)) => {
            println!("Start Filtering!");
            activations = converter::filter::filter(&activations, min, max, keep_value as f64);
        },
        _ => {},
    }

    println!("Start Writing!(This may take a while)");
    converter::writer::to_w2v(&activations, &output);

    println!("Finish Writing!");
}

#[pyfunction]
fn version() -> String {
    format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[pymodule]
fn concept_x_converter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(processor, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;

    Ok(())
}