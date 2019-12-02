pub mod processor;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("Please enter the intcode program:");

    // Read in the line
    let mut buffer = String::new();
    match stdin.read_line(&mut buffer) {
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
    let results: (u32, u32) = brute_force_expected_result(stack);
    println!("Required values are {}, {}", results.0, results.1);

    // Build & display the output
    // let mut output_string = String::new();
    // for value in stack {
    //     let value_string: String = value.to_string();
    //     output_string.push_str(&value_string);
    //     output_string.push(',')
    // }

    // output_string.pop();
    // println!("Modified stack:");
    // println!("{}", output_string);


    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn brute_force_expected_result(stack: Vec<u32>) -> (u32, u32) {
    for x in 0..99 {
        for y in 0..99 {
            let mut memory: Vec<u32> = stack.to_vec();
            memory[1] = x;
            memory[2] = y;

            processor::run_program(&mut memory);

            if memory[0] == 19690720 {
                return (x, y);
            }
        }
    }

    panic!("Could not find values producing expected result.");
}