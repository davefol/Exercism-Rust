pub fn nth(n: u32) -> u32 {
    // brute force
    
    let mut primes_found = 1;
    let mut test_number:u32 = 3;


    while primes_found < n + 1 {
        let mut is_composite = false;
        for i in 2..test_number-1 {
            if test_number % i == 0 {
                is_composite = true;
                break;
            }
        }
        if is_composite == false {
            primes_found = primes_found + 1;
        }
        test_number = test_number + 1;
    }
    return test_number - 1;
}
