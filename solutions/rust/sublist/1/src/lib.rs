#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if is_sublist(first_list, second_list) {
        Comparison::Sublist
    } else if is_superlist(first_list, second_list) {
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

fn is_superlist(first_list: &[i32], second_list: &[i32]) -> bool {
    match (first_list, second_list) {
        ([_, ..], []) => true,
        _ => first_list
            .windows(second_list.len())
            .any(|window| window == second_list),
    }
}

fn is_sublist(first_list: &[i32], second_list: &[i32]) -> bool {
    match (first_list, second_list) {
        ([], [_, ..]) => true,
        _ => second_list
            .windows(first_list.len())
            .any(|window| window == first_list),
    }
}
