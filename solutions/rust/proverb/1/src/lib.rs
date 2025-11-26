pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut rhymes: Vec<String> = list
        .windows(2)
        .map(|items| format!("For want of a {} the {} was lost.", items[0], items[1]))
        .collect();

    rhymes.push(format!("And all for the want of a {}.", list[0]));

    rhymes.join("\n")
}
