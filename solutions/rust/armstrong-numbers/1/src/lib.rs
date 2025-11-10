pub fn is_armstrong_number(num: u32) -> bool {
    let stringified_num = num.to_string();
    let number_of_digits = stringified_num.len() as u32;
    let armstrong_number: u32 = num
        .to_string()
        .bytes()
        .map(|bytes| (bytes - 48u8) as u32)
        .map(|digit| digit.pow(number_of_digits))
        .sum();

    num == armstrong_number
}
