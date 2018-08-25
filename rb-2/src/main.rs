extern crate primal;

use primal::Sieve;

fn main() {
    // sieve with an upper bound of 10000
    let sieve = Sieve::new(10000);

    // detect if a number is or is not prime
    let suspect = 5273;
    println!("{} is print: {}", suspect, sieve.is_prime(suspect));
    let not_prime = 1024;
    println!("{} is print: {}", suspect, sieve.is_prime(not_prime));

    // find the 1000th prime number
    let n = 1000;
    match sieve.primes_from(0).nth(n-1) {
        Some(number) => println!("{}th prime is {}", n, number),
        None => println!("I just don't know about {}th prime", n),
    }

    // Factorization

    // number of divisors
    println!("{:?}", num_divisors(2610, &sieve));
    println!("{:?}", num_divisors(420, &sieve));
    println!("{:?}", num_divisors(720720, &sieve));
    println!("{:?}", num_divisors(55440, &sieve));

    // all prime divisors and their occurrences
    // result Ok([(2, 1), (3, 2), (5, 1), (29, 1)]) == (2 3 3 5 29)
    println!("{:?}", sieve.factor(2610));
}

// calculate all of the numbers that can divide into a number
// division ,must end in a whole number
fn num_divisors(n: usize, primes: &Sieve) -> Option<usize> {
    // match only numbers that are factors of the number passed in
    match primes.factor(n) {
        Ok(factors) => Some(factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1))),
        Err(_) => None,
    }
}
