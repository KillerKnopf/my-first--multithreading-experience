use std::{
    fmt::Display,
    time::{Duration, Instant},
};

// Module where I put the implementations of the prime generators
pub mod prime_generators;

// The struct holds all generated data of the program
struct AppState {
    // The user choosen limit until which prime numbers are searched for.
    limit: u32,
    // The result of the prime generation from the primal crate.
    baseline: PrimeResult,
    // List of the results of my prime generating functions.
    my_results: Vec<PrimeResult>,
}

// Struct which holds the prime generator version, the found primes and the elapsed time of the prime generator
#[derive(Debug, Default)]
struct PrimeResult {
    identifier: String,
    primes: Vec<u32>,
    elapsed_time: Duration,
}

// TODO
// Used when writing results to console (or maybe a file)
impl Display for PrimeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {}

// This function wraps the prime generating functions so that their runtime is measured.
// A prime generating function takes an u32 as the only argument (the limit until which prime numbers are searched).
// It returns a vec of u32s which contains all found primes.
// An identifier is used to determine which prime generator was used.
fn run_prime_generator(
    prime_generator: fn(u32) -> Vec<u32>,
    limit: u32,
    identifier: &'static str,
) -> PrimeResult {
    // Take timestamp
    let start = Instant::now();

    // Run passed function with limit as it's argument. This will return a vec<u32>.
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
