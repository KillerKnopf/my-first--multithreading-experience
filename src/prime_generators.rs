use primal::Sieve;

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
