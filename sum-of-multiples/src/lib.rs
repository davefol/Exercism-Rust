use std::collections::HashSet;
use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter()
        .flat_map(|x| multiples(*x, limit))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}

fn multiples(factor: u32, max: u32) -> Vec<u32> {
    (0..u32::max_value())
        .take_while(|x| factor * x < max)
        .map(|x| factor * x)
        .collect::<Vec<u32>>()
}

pub fn sum_of_multiple(limit: u32, factor: u32) -> u32 {
    let n = (limit as f32 / factor as f32).floor() as u32;
    factor * n * (n+1) / 2
}

pub fn sum_of_multiple_brute(limit: u32, factor: u32) -> u32 {
    multiples(factor, limit)
        .iter()
        .sum()
}

pub fn prime_factorization(factor: u32) -> Vec<u32> {
    let mut prime_factors = Vec::<u32>::new();
    
    let mut n = factor;

    while n % 2 == 0 {
        prime_factors.push(2);
        n /=  2;
    }

    for i in (3..(n as f32).sqrt() as u32 + 1).step_by(2) {
        while n % i == 0 {
            prime_factors.push(i);
            n /= 2;
        }
    }

    if n > 2 {
        prime_factors.push(n);
    }
    
    prime_factors
}
