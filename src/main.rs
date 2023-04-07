mod converter;

fn main() {
    // read read args
    let args: Vec<String> = std::env::args().collect();
    let input_file_name = &args[1];
    let output_file_name = &args[2];

    println!("Start Reading! (This may take a while)");
    let activations = converter::reader::Activation::from_file(&input_file_name);
    println!("Finish Reading! There are {} activations", activations.len());

    println!("Start Writing!(This may take a while)");
    converter::writer::to_w2v(&activations, &output_file_name);
    println!("Finish Writing!");
}
