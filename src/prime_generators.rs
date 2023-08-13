use primal::Sieve;
use std::thread;

pub fn generate_baseline(limit: usize) -> Vec<usize> {
    // Create a primal Sieve which generates all primes until inclusive the passed limit
    let sieve = Sieve::new(limit);

    // Using an iterator take all primes from it and collect them into an Vec<usize> which gets returned
    // Also limit was used to limit the amount of primes sometimes this include primes alightly above it
    // Take_while is used to properly enforce the limit
    sieve.primes_from(0).take_while(|p| p < &limit).collect()
}

pub fn generate_primes_v1_0(limit: usize) -> Vec<usize> {
    // No primes are smaller than 2
    if limit < 2 {
        return Vec::new();
    }

    // If limit at least 2 than 2 will always be included so vec initialized with 2
    let mut primes: Vec<usize> = vec![2];

    // Find the rest of the primes by iterating over all numbers
    // and dividing them over all previous numbers.
    for number in 3..limit {
        let mut is_prime = true;

        for divider in 2..number {
            // If remainder of divison from number and divider is 0 than number is not prime
            // Break because remaining operations don't matter
            if number % divider == 0 {
                is_prime = false;
                break;
            }
            // They only divider that can evenly divide number after divider is largen than a third of number
            // is the half of number which won't change the outcome
            // An even number except for 2 can't be prime and would already have been tested against 2
            // An odd number is not evenly divisble by it's half and won't change the result
            if divider * 3 > number {
                break;
            }
        }

        if is_prime {
            primes.push(number);
        }
    }

    primes
}

pub fn generate_primes_v1_1(limit: usize) -> Vec<usize> {
    // No primes are smaller than 2
    if limit < 2 {
        return Vec::new();
    }

    // If limit at least 2 than 2 will always be included so vec initialized with 2
    let mut primes: Vec<usize> = vec![2];

    // Find the rest of the primes by iterating over all numbers
    // and dividing them over all previous numbers.
    for number in 3..limit {
        let mut is_prime = true;

        if number % 2 == 0 {
            continue;
        }

        for divider in (3..number).step_by(2) {
            // If remainder of divison from number and divider is 0 than number is not prime
            // Break because remaining operations don't matter
            if number % divider == 0 {
                is_prime = false;
                break;
            }
            // They only divider that can evenly divide number after divider is largen than a third of number
            // is the half of number which won't change the outcome
            // An even number except for 2 can't be prime and would already have been tested against 2
            // An odd number is not evenly divisble by it's half and won't change the result
            if divider * 3 > number {
                break;
            }
        }

        if is_prime {
            primes.push(number);
        }
    }

    primes
}

pub fn generate_primes_v2_0(limit: usize) -> Vec<usize> {
    // No primes are smaller than 2 so no work to do
    if limit < 2 {
        return Vec::new();
    }
    // If limit is 2 then just return vec with 2
    if limit == 2 {
        return vec![2];
    }

    // Number of threads that will be spawned
    let thread_count = 10;
    // Size of each chunk. Float to calculate properly
    let step_size = limit as f64 / thread_count as f64;

    // Struct that defines the ranges the thread will process
    struct SearchRange {
        start: usize,
        end: usize,
    }

    // Vec to store the ranges
    let mut search_ranges: Vec<SearchRange> = Vec::new();

    // Start and end values of the ranges each thread will process
    // Floats for proper calculating and even distribution
    let mut start_float = 3.0;
    let mut end_float = start_float + step_size;

    for _ in 0..10 {
        // Cast float range limits into usize so that they can be used
        let start = start_float as usize;
        let mut end = end_float as usize;
        // Too make sure the algorithm doesn't overshoot the target limit with the last thread
        if end > limit {
            end = limit;
        }

        // Construct SearchRange and store it
        search_ranges.push(SearchRange { start, end });

        // Updating variables for next thread
        // Set start to previous end
        start_float = end_float;
        // Increment end
        end_float += step_size;
    }

    let mut primes: Vec<usize> = Vec::new();

    // Spawn a thread for each SearchRange
    thread::scope(|scope| {
        for search_range in &search_ranges {
            let prime_list = scope.spawn(|| {
                let mut primes: Vec<usize> = Vec::new();
                // Identify first thread and have it add the 2
                if search_range.start == 3 {
                    primes.push(2);
                }

                for number in search_range.start..search_range.end {
                    let mut is_prime = true;

                    if number % 2 == 0 {
                        continue;
                    }

                    for divider in (3..number).step_by(2) {
                        // If remainder of divison from number and divider is 0 than number is not prime
                        // Break because remaining operations don't matter
                        if number % divider == 0 {
                            is_prime = false;
                            break;
                        }
                        // They only divider that can evenly divide number after divider is largen than a third of number
                        // is the half of number which won't change the outcome
                        // An even number except for 2 can't be prime and would already have been tested against 2
                        // An odd number is not evenly divisble by it's half and won't change the result
                        if divider * 3 > number {
                            break;
                        }
                    }

                    if is_prime {
                        primes.push(number);
                    }
                }

                primes
            });

            (&mut primes).append(&mut prime_list.join().unwrap());
        }
    });

    // // DEBUG print
    // for (i, prime) in primes.iter().enumerate() {
    //     print!(" {} ", prime);
    //     if i % 8 == 7 {
    //         println!();
    //     }
    // }
    // println!();

    primes
}
