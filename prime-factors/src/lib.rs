pub fn factors(n: u64) -> Vec<u64> {
    println!("finding prime factors for n = {}.", n);
    let mut divide_out = n;
    let mut prime_factors = Vec::<u64>::new();

    let size: usize = (n+3) as usize;
    let mut sieve = vec![true;size];
    sieve[0] = false;
    sieve[1] = false;

    let root = (size as f64).sqrt().ceil() as u64;

    for number in 2..root {
        if !sieve[number as usize]  {
            continue;
        }

        while divide_out % number == 0 {
            println!("found factor {}.", number);
            prime_factors.push(number);
            divide_out /= number;
        }

        let mut multiple = number * number;
        while multiple < size as u64 {
            println!("{}/{} processed.", multiple, n);
            sieve[multiple as usize] = false;
            multiple += number;
        }
    }

    for number in (root + 1)..size as u64 {
        while sieve[number as usize] && divide_out % number == 0 {
            println!("found factor {}.", number);
            prime_factors.push(number);
            divide_out /= number;
        }
    }

    prime_factors
}

