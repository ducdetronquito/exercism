pub fn factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    }
    if n == 2 {
        return vec![2];
    }
    if n == 3 {
        return vec![3];
    }

    let mut result = vec![];
    let mut remainder = n;
    let upper_bound = n.div_euclid(2);
    for i in 2..=upper_bound {
        if !is_prime(i) {
            continue;
        }
        while remainder.rem_euclid(i) == 0 {
            result.push(i);
            remainder = remainder.div_euclid(i);
        }
    }

    result
}

fn is_prime(value: u64) -> bool {
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
