use flate2::read::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufReader, Write};

#[derive(Deserialize)]
struct Reference {
    vector: [f32; 14],
    label: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "resources/references.json.gz";
    let output_path = "resources/references.bin";

    println!("Reading {}...", input_path);
    let file = File::open(input_path)?;
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);

    let references: Vec<Reference> = serde_json::from_reader(reader)?;
    println!("Loaded {} records.", references.len());

    let mut output = File::create(output_path)?;

    for (_i, ref_data) in references.iter().enumerate() {
        for &val in &ref_data.vector {
            let quantized = (((val + 1.0) / 2.0) * 65535.0).round().clamp(0.0, 65535.0) as u16;
            output.write_all(&quantized.to_le_bytes())?;
        }

        let label_byte = if ref_data.label == "fraud" { 1u8 } else { 0u8 };
        output.write_all(&[label_byte])?;
    }

    println!("Sucess! Binary file saved to {}", output_path);
    println!(
        "Final size: {} bytes",
        std::fs::metadata(output_path)?.len()
    );

    Ok(())
}
