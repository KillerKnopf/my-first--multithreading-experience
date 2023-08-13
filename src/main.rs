use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    process,
    time::{Duration, Instant},
};

// Module where I put the implementations of the prime generators
pub mod prime_generators;

// The struct holds all generated data of the program.
#[derive(Default)]
struct AppState {
    // The user chosen limit until which prime numbers are searched for.
    limit: usize,
    // The result of the prime generation from the primal crate.
    baseline: PrimeResult,
    // List of the results of my prime generating functions.
    my_results: Vec<PrimeResult>,
}

// Struct which holds the prime generator version, the found primes and the elapsed time of the prime generator
#[derive(Debug, Default)]
struct PrimeResult {
    identifier: String,
    primes: Vec<usize>,
    elapsed_time: Duration,
}

// TODO
// Used when writing results to console (or maybe a file)
impl Display for PrimeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    // Initialize AppState
    let mut app_state = AppState::default();

    // Write main menu to console
    write_main_menu();

    // Get user input for limit, validate and store in appState
    // Loops so that failed input results in retrys
    loop {
        // Get user input
        let mut raw_input = String::new();
        stdin().read_line(&mut raw_input).unwrap();

        // Parse user input into usize
        // Trim input beforehand because it contains whitespace (probably a line break at the end)
        match raw_input.trim().parse::<usize>() {
            Ok(value) => {
                app_state.limit = value;
                break;
            }
            Err(_) => {
                println!("\n No valid number found. Please try again.");
                println!(" Press enter to continue");
                stdin().read_line(&mut String::new()).unwrap();
            }
        }
        write_main_menu();
    }

    // Run baseline and store result in appState
    app_state.baseline = run_prime_generator(
        prime_generators::generate_baseline,
        app_state.limit,
        "baseline",
    );

    // Run my prime generators and store results in appState

    // Write results to console
    println!("\n");
}

// This function wraps the prime generating functions so that their runtime is measured.
// A prime generating function takes an usize as the only argument (the limit until which prime numbers are searched).
// It returns a vec of usizes which contains all found primes.
// An identifier is used to determine which prime generator was used.
fn run_prime_generator(
    prime_generator: fn(usize) -> Vec<usize>,
    limit: usize,
    identifier: &'static str,
) -> PrimeResult {
    // Take timestamp
    let start = Instant::now();

    // Run passed function with limit as it's argument. This will return a vec<usize>.
    let primes = prime_generator(limit);

    // Calculate elapsed time
    let elapsed_time = start.elapsed();

    // Construct PrimeResult
    PrimeResult {
        identifier: identifier.to_string(),
        primes,
        elapsed_time,
    }
}

fn write_main_menu() {
    // Clearing the console using the ClearScreen crate
    clearscreen::clear().expect("Failed to clear terminal");

    // Writing messages to the emptied console
    println!(
        "\n This program runs multiple algorithm to generate prime numbers and benchmarks them."
    );
    println!(
        " -----------------------------------------------------------------------------------\n"
    );
    println!(
        " To run the benchmarks please enter the limit until which the prime numbers are generated."
    );
    println!(" This limit is inclusive. You can choose a number between 0 and 18'446'744'073'709'551'615.");

    // Using print!() to write input prompt so that the user input is written immediately on the same line.
    print!("\n Your chosen limit --> ");

    // Flushing stdout because print!() does not flush unlike println!().
    // Flushing in this case means writing the buffer (String to write) to the console.
    // If this flush is not done then other terminal stuff may happen before like input.
    // That means the user gets shown "Your chosen limit -> " after the user entered some input.
    stdout().flush().unwrap();
}
