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

        "day01b" => println!(
            "{}",
            aoc21::day01::num_larger_measurement_windows(
                aoc21::utils::lines_as_i64_vec(&input).as_slice()
            )
        ),

        "day02a" => println!("{}", {
            let (h, d) = aoc21::day02::location_after_commands(
                aoc21::day02::lines_as_command_vec(&input).as_slice(),
            );
            h * d
        }),

        "day02b" => println!("{}", {
            let (h, d) = aoc21::day02::aimed_location_after_commands(
                aoc21::day02::lines_as_command_vec(&input).as_slice(),
            );
            h * d
        }),

        _ => println!("{}: unknown command", command),
    }
}
