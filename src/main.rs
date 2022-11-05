use std::env;
use std::io;
use std::process;

fn main() {
    let Some(command) = env::args().nth(1) else {
        println!("Usage: aoc21 <dayNNx>");
        process::exit(1);
    };

    let Ok(input) = io::read_to_string(io::stdin()) else {
        println!("Failed to read input from stdin");
        process::exit(2);
    };

    match command.as_ref() {
        "day01a" => println!(
            "{}",
            aoc21::day01::num_larger_measurements(
                aoc21::utils::lines_as_i64_vec(&input).as_slice()
            )
        ),

        _ => println!("{}: unknown command", command),
    }
}