pub fn is_valid(code: &str) -> bool {
    let Ok(digits) = as_digits(code) else {
        return false;
    };

    let checksum: i32 = digits
        .iter()
        .enumerate()
        .map(|(i, value)| {
            if (i + 1).rem_euclid(2) == 0 {
                let double = *value * 2;
                if double > 9 { double - 9 } else { double }
            } else {
                *value
            }
        })
        .sum();

    checksum.rem_euclid(10) == 0
}

fn as_digits(code: &str) -> Result<Vec<i32>, &str> {
    let values: Vec<i32> = code
        .replace(" ", "")
        .bytes()
        .rev()
        .map(|value| value as i32 - 48)
        .collect();

    if values.len() <= 1 {
        return Err("Too short");
    }

    if values.iter().any(|number| *number < 0 || *number >= 10) {
        return Err("Contains non-digit values");
    }

    Ok(values)
}
