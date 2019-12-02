// Apparently this only needs to exist in the "main" module
// Note: Unless an external dependency is referenced like such, attempting to 'use' it will result in
// annoying and unhelpful "unresolved import `relative_path`" errors.
extern crate math;

use std::env;
use std::path::{PathBuf};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub mod mass_calculator;

fn main() {
    // Figure out the path to the data file
    let path_to_data_file: PathBuf = get_data_file_path();

    // Start iterating it and summing fuel values
    let mut total_fuel: i32 = 0;

    let file = File::open(path_to_data_file).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mass_value: i32 = line.unwrap().parse().unwrap();
        let fuel_value: i32 = mass_calculator::get_fuel_required_for_mass(mass_value);
        total_fuel += fuel_value;
    }

    // Output the total
    println!("Total fuel required: {}", total_fuel);
}

fn get_data_file_path() -> PathBuf {
    let mut current_dir: PathBuf = env::current_dir().unwrap();
    println!("Current directory: {}", current_dir.display());

    current_dir.push("data\\mass_values.txt");
    println!("Data file path: {}", current_dir.display());

    return current_dir;
}