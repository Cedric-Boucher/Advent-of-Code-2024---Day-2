use std::{error::Error};

fn import_data(filepath: &str) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
    let mut data: Vec<Vec<u32>> = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(filepath)
        .unwrap();

    for result in reader.records() {
        let record = result?;
        let row: Vec<u32> = record.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        data.push(row);
    }

    Ok(data)
}

fn main() {
    // get file path from command line argument

    // I replaced all spaces with commas in the input file to make it easier to parse.

    let file_path: String = std::env::args().nth(1).expect("Please provide a file path as an argument");

    let data: Vec<Vec<u32>> = import_data(&file_path).unwrap();

    println!("Data: {:?}", data);
}
