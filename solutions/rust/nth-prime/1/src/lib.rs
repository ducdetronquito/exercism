pub fn nth(n: u32) -> u32 {
    let mut primes_count = 0;
    let mut i = 0;
    loop {
        if is_prime(i) {
            if primes_count == n {
                return i;
            }
            primes_count += 1;
        }
        i += 1;
    }
}

fn is_prime(value: u32) -> bool {
    if value <= 1 || value == 4 {
        return false;
    }
    if value == 2 || value == 3 || value == 5 {
        return true;
    }

    let upper_bound = value.div_euclid(2);

    for i in 2..=upper_bound {
        if value.rem_euclid(i) == 0 {
            return false;
        }
    }
    true
}
