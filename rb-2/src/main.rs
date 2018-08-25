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
    println!("{:?}", sieve.factor(2610));
}