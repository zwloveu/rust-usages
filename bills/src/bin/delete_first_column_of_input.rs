use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = File::open("input.csv")?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(input_file);

    // Convert headers to mutable Vec<String> and remove first column
    let mut headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();
    headers.remove(0); // Now works on Vec

    let mut records = Vec::new();
    for result in reader.records() {
        // Convert record to owned Vec<String> to avoid temporary lifetime issues
        let mut record: Vec<String> = result?
            .iter()
            .map(|s| s.to_string()) // Convert &str to owned String
            .collect();
        record.remove(0); // Now works with owned values
        records.push(record);
    }

    let output_file = File::create("input.csv")?;
    let mut writer = WriterBuilder::new().from_writer(output_file);

    writer.write_record(&headers)?;
    for record in records {
        writer.write_record(&record)?;
    }

    writer.flush()?;
    println!("deleted first column of input.csv");
    Ok(())
}
