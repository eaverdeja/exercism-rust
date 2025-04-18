pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| {
            let bottles = start_bottles - i;
            [
                first_verse(bottles),
                first_verse(bottles),
                second_verse(),
                third_verse(bottles - 1),
            ]
            .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn first_verse(num_of_bottles: u32) -> String {
    if num_of_bottles == 1 {
        return String::from("One green bottle hanging on the wall,");
    }
    let num_name = to_digit_name(num_of_bottles, true);
    format!("{num_name} green bottles hanging on the wall,")
}

fn second_verse() -> String {
    String::from("And if one green bottle should accidentally fall,")
}

fn third_verse(num_of_bottles: u32) -> String {
    if num_of_bottles == 0 {
        return String::from("There'll be no green bottles hanging on the wall.");
    }
    if num_of_bottles == 1 {
        return String::from("There'll be one green bottle hanging on the wall.");
    }
    let num_name = to_digit_name(num_of_bottles, false);
    format!("There'll be {num_name} green bottles hanging on the wall.")
}

fn to_digit_name(num: u32, uppercase: bool) -> String {
    let mut name = match num {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        _ => panic!("Only supports digits 1-10"),
    };
    if uppercase {
        let mut c = name.chars();
        name = match c.next() {
            None => String::new(),
            Some(first_char) => first_char.to_uppercase().chain(c).collect(),
        };
    }
    name
}
