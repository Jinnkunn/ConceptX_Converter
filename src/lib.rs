use pyo3::prelude::*;

mod converter;

#[pyfunction]
fn processor(input_file_name: &str, output_file_name: &str) {
    // read read args
    // let args: Vec<String> = std::env::args().collect();
    // let input_file_name = &args[1];
    // let output_file_name = &args[2];

    println!("Start Reading! (This may take a while)");
    let activations = converter::reader::Activation::from_file(&input_file_name);
    println!("Finish Reading! There are {} activations", activations.len());

    println!("Start Writing!(This may take a while)");
    converter::writer::to_w2v(&activations, &output_file_name);
    println!("Finish Writing!");
}

#[pymodule]
fn concept_x_converter(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(processor, m)?)?;

    Ok(())
}