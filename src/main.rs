use std::{error::Error};

#[derive(Debug, Clone)]
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

    fn without_level(&self, index: usize) -> Self {
        if index < self.levels.len() {
            let mut report_copy = self.clone();
            report_copy.levels.remove(index);
            report_copy
        }
        else {
            self.clone().into()
        }
    }

    fn is_safe_with_problem_dampener(&self) -> bool {
        // A report is considered safe if both of the following are true:
        // - the levels are either all increasing or all decreasing.
        // - any two adjacent levels differ by at least one and at most three.
        // If removing a single level from an unsafe report would make it safe,
        // the report counts as safe with the problem dampener.
        if self.is_safe() {
            true
        } else {
            // Iterate through each level and check if removing it would make the report safe.
            self.levels.iter().enumerate().any(|(i, _)| {
                self.clone().without_level(i).is_safe()
            })
        }
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

    // Part 1: Count number of safe reports.

    let safe_count: usize = (&data).into_iter().filter(|report| report.is_safe()).count();

    println!("Number of safe reports: {}", safe_count);

    // Part 2: Count number of reports that are safe with the problem dampener.

    let safe_with_problem_dampener_count: usize = (&data).into_iter().filter(|report| report.is_safe_with_problem_dampener()).count();

    println!("Number of reports that are safe with the problem dampener: {}", safe_with_problem_dampener_count);
}
