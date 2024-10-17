use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use big_rational::BigRational;
use num_bigint::{BigUint, RandomBits};
use rand::{rngs::ThreadRng, thread_rng, Rng};
pub mod big_rational;

fn main() {
    let count = 10;
    println!("count = {}", count);
    let k_values: [u64; 8] = [50, 100, 150, 200, 250, 300, 350, 400];
    let mut rng = thread_rng();
    for k in k_values.iter() {
        let randoms = generate_randoms(&mut rng, count, *k);
        let mut durations: Vec<Duration> = Vec::new();
        println!("k = {}", k);
        println!("n / phi(n) / duration (s)");
        for n in &randoms {
            print!("{:?}\t", n);
            let _ = io::stdout().flush();
            let start = Instant::now();
            let factors = prime_factors(&n);
            let phi = calc_phi(&n, &factors);
            let duration = Instant::now() - start;
            durations.push(duration);
            println!("{:?}\t{:.2}s", phi, duration.as_secs_f32());
        }
        let s: f64 = durations.iter().map(|d| d.as_secs_f64()).sum();
        let avg = s / durations.len() as f64;
        println!("Average time in seconds: {:.2}", avg);
    }
}

fn prime_factors(n: &BigUint) -> Vec<BigUint> {
    let mut n = n.clone();
    let two: BigUint = 2u8.into();
    let max_factor_count = n.sqrt();
    let mut factors: Vec<BigUint> = Vec::new();
    if &n % &two == BigUint::ZERO {
        factors.push(two.clone());
        n >>= 1;
        while &n % &two == BigUint::ZERO {
            n >>= 1;
        }
    }

    let mut i: BigUint = 3u8.into();
    while &i < &max_factor_count {
        if &n % &i == BigUint::ZERO {
            n /= &i;
            while &n % &i == BigUint::ZERO {
                n /= &i;
            }
            factors.push(i.clone());
        }
        i += &two;
    }
    if &n > &two {
        factors.push(n);
    }
    return factors;
}

fn calc_phi(n: &BigUint, factors: &Vec<BigUint>) -> BigUint {
    let mut product: BigRational = n.clone().into();
    for factor in factors.iter() {
        let denominator = factor;
        product *= BigRational::new(denominator - BigUint::from(1u8), denominator.clone());
    }
    return product.into();
}

fn generate_randoms(rng: &mut ThreadRng, count: usize, k: u64) -> Vec<BigUint> {
    let mut rands: Vec<BigUint> = Vec::new();
    for _ in 1..count {
        let rand = rng.sample(RandomBits::new(k));
        rands.push(rand);
    }
    return rands;
}
