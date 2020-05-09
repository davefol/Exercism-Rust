pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (upper_bound+1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 0..=((upper_bound as f64).sqrt() as u64) {
        if is_prime[i as usize] {
            for j in ((i as u64).pow(2)..=upper_bound).step_by(i as usize) {
                is_prime[j as usize] = false;
            }
        }
    }
    is_prime.iter().enumerate().filter(|(i, x)| is_prime[*i]).map(|(i, x)| i as u64).collect()
}
