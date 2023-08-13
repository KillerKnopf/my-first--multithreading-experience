use primal::Sieve;

pub fn generate_baseline(limit: usize) -> Vec<usize> {
    // Create a primal Sieve which generates all primes until inclusive the passed limit
    let sieve = Sieve::new(limit);

    // Using an iterator take all primes from it and collect them into an Vec<usize> which gets returned
    // Also limit was used to limit the amount of primes sometimes this include primes alightly above it
    // Take_while is used to properly enforce the limit
    sieve.primes_from(0).take_while(|p| p < &limit).collect()
}
