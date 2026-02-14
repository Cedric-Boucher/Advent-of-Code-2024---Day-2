use std::{error::Error};

#[derive(Debug)]
struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn new(levels: Vec<u32>) -> Self {
        Report { levels }
    }

    fn is_safe(&self) -> bool {
        // A report is considered safe if both of the following are true:
        // - the levels are either all increasing or all decreasing.
        // - any two adjacent levels differ by at least one and at most three.
        let increasing: bool = self.levels.iter().zip(self.levels.iter().skip(1)).all(|(a, b)| a < b);
        let decreasing: bool = self.levels.iter().zip(self.levels.iter().skip(1)).all(|(a, b)| a > b);
        let adjacent_diff: bool = self.levels.iter().zip(self.levels.iter().skip(1)).all(|(a, b)| {
            let diff = if a > b { a - b } else { b - a };
            diff >= 1 && diff <= 3
        });

        (increasing || decreasing) && adjacent_diff
    }
}

fn import_data(filepath: &str) -> Result<Vec<Report>, Box<dyn Error>> {
    let mut data: Vec<Report> = Vec::new();

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .has_headers(false)
        .from_path(filepath)
        .unwrap();

    for result in reader.records() {
        let record = result?;
        let row: Vec<u32> = record.iter().map(|s| s.parse::<u32>().unwrap()).collect();
        data.push(Report::new(row));
    }

    Ok(data)
}

fn main() {
    // get file path from command line argument

    // I replaced all spaces with commas in the input file to make it easier to parse.

    let file_path: String = std::env::args().nth(1).expect("Please provide a file path as an argument");

    let data: Vec<Report> = import_data(&file_path).unwrap();

    let safe_count: usize = data.into_iter().filter(|report| report.is_safe()).count();

    println!("Number of safe reports: {}", safe_count);
}
