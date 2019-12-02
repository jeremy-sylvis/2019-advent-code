pub mod processor;

use std::io::stdin;

fn main() {
    println!("Please enter the intcode program:");

    // Read in the line
    let mut buffer = String::new();
    match stdin().read_line(&mut buffer) {
        Ok(n) => println!("Read {} bytes.", n),
        Err(error) => println!("Encountered error: {}", error)
    };
    
    // Split & iterate
    let segments = buffer.trim_end().split(',');
    let mut stack: Vec<u32> = Vec::new();

    for segment in segments {
        let intcode: u32 = segment.parse().unwrap();
        stack.push(intcode);
    }

    // Run the app
    processor::run_program(&mut stack);

    // Build & display the output
    let mut output_string = String::new();
    for value in stack {
        let value_string: String = value.to_string();
        output_string.push_str(&value_string);
        output_string.push(',')
    }

    output_string.pop();
    println!("Modified stack:");
    println!("{}", output_string);
}