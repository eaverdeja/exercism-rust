const CHILDREN: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let start = CHILDREN
        .iter()
        .position(|&child| child == student)
        .expect("Could not find child!")
        * 2;
    let end = start + 2;

    let to_seed_name = |label: char| match label {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("Unknown seed!"),
    };

    diagram
        .lines()
        .flat_map(|line| line.trim()[start..end].chars().map(to_seed_name))
        .collect()
}
