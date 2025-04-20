#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    } else if first_list.len() == second_list.len() {
        return Comparison::Unequal;
    }

    let (smaller, bigger, comparison) = if first_list.len() < second_list.len() {
        (first_list, second_list, Comparison::Sublist)
    } else {
        (second_list, first_list, Comparison::Superlist)
    };
    if smaller.is_empty() {
        return comparison;
    }
    if bigger
        .windows(smaller.len())
        .any(|window| window == smaller)
    {
        return comparison;
    }

    Comparison::Unequal
}
