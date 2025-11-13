pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let verses: Vec<String> = (0..=start_bottles)
        .rev()
        .take(take_down as usize)
        .map(verse)
        .collect();

    verses.join("\n\n")
}

fn verse(number_of_bottles: u32) -> String {
    let mut verse = first_sentence(number_of_bottles).repeat(2);
    verse.push_str("And if one green bottle should accidentally fall,\n");
    verse.push_str(&last_sentence(number_of_bottles));
    verse
}

fn first_sentence(number_of_bottles: u32) -> String {
    let bottles = if number_of_bottles > 1 {
        "bottles"
    } else {
        "bottle"
    };
    format!(
        "{} green {bottles} hanging on the wall,\n",
        as_word(number_of_bottles),
    )
}

fn last_sentence(number_of_bottles: u32) -> String {
    let next_number_of_bottles = number_of_bottles - 1;
    let bottles = if (next_number_of_bottles) == 1 {
        "bottle"
    } else {
        "bottles"
    };
    format!(
        "There'll be {} green {bottles} hanging on the wall.",
        as_word(next_number_of_bottles).to_lowercase()
    )
}

fn as_word(number_of_bottles: u32) -> &'static str {
    match number_of_bottles {
        10 => "Ten",
        9 => "Nine",
        8 => "Eight",
        7 => "Seven",
        6 => "Six",
        5 => "Five",
        4 => "Four",
        3 => "Three",
        2 => "Two",
        1 => "One",
        0 => "No",
        _ => panic!("Unexpected number of bottles"),
    }
}
