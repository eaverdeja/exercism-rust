use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let pairs = HashMap::from([(']', '['), ('}', '{'), (')', '(')]);
    let mut stack = Vec::new();

    for char in string.chars() {
        match char {
            '[' | '{' | '(' => stack.push(char),
            ']' | '}' | ')' => {
                // Does the most recent opening bracket match
                // this closing bracket?
                if let Some(last_opening) = stack.pop() {
                    if let Some(&expected_opening) = pairs.get(&char) {
                        if last_opening != expected_opening {
                            return false;
                        }
                    }
                } else {
                    // This means we have a closing bracket before
                    // an opening bracket, which is illegal
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.is_empty()
}
