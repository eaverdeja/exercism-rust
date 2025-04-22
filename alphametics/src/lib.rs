/*
This solution branches into two different approaches.
It completes the entire test suite in ~1s, give or take.

The first one is used for most puzzles and is more straightforward to follow.
The second one is used for very large puzzles and uses a more complex strategy.

These different approaches were kept as they optimize
for different things (ease of comprehension vs performance)

Both strategies will assign digits to each of the letters in the puzzle,
creating a map of potential "candidates". We recursively backtrack
through these candidates, evaluating the equation when possible
to check for solutions. What differs is how we evaluate the equation.

1. Sum up the left-hand side and compare it to the right-hand side

The idea here is straightforward. Sum everybody on the LHS and check if
it's equal to the RHS. Duh.

It will attempt to do partial validation of the least significant columns
in order to prune bad candidates early on. This has a notable performance
boost for average-sized puzzles.

When the candidate map is fully assigned, the equation is evaluated
and checked for truthiness. If it does evaluate successfully, the
candidate is picked as a solution. If it does not evaluate successfully,
we backtrack by removing the latest assignment from the candidate map
and trying another one.

This strategy has factorial complexity from the permutation search,
multiplied by O(n) for each evaluation, where n is the number of
addends in the equation. For large puzzles with many addends, this
multiplier becomes significant and the solution melts down 🫠

2. Dot product of digit-letter weights

You can tell by the heading that this solution is more complicated.

Instead of evaluating each word during each permutation trial,
we precompute how much each letter contributes to the overall equation.

For each letter in an addend word on the left-hand side,
its weight depends on its position:
- Units place: +1
- Tens place: +10
- Hundreds place: +100
- etc.

For letters in the result (right-hand side), we use negative weights:
- Units place: -1
- Tens place: -10
- etc.

For a solution to be valid, the dot product of these weights
with assigned digit values must equal zero.

Ok... let's check out an example to see what the hell this means.

"SEND + MORE == MONEY"
Let's trace the weight calculation for 'E':

In "SEND": 'E' is in hundreds place = +100
In "MORE": 'E' is in units place = +1
In "MONEY": 'E' is in tens place = -10
Total weight for 'E' = 100 + 1 - 10 = 91

For all letters:
S: +1000 (thousands place in SEND)
E: +91 (as calculated above)
N: -90 (negative tens in MONEY, positive hundreds in SEND)
D: +1 (units in SEND)
M: -9000 (negative ten-thousands in MONEY, positive thousands in MORE)
O: -900 (negative hundreds in MONEY, positive hundreds in MORE)
R: +10 (tens in MORE)
Y: -1 (units in MONEY)

When we plug in the known solution (S=9, E=5, N=6, D=7, M=1, O=0, R=8, Y=2):
`1000*9 + 91*5 - 90*6 + 1*7 - 9000*1 - 900*0 + 10*8 - 1*2 = 0`

*mindblown* 🤯

This strategy has factorial complexity from the permutation search,
multiplied by O(n) for each evaluation, where n is the *number of
distinct letters in the puzzle*.

For large puzzles this is the way to go 🚀

This alternative solution was inspired by user alterpatzer in:
https://forum.exercism.org/t/alphametics-exercise-need-help-with-performance/3614
*/

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    iter::{self},
};

struct Value(String);

struct Expression(Vec<Value>);

struct Equation {
    lhs: Expression,
    rhs: Value,
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value(value.trim().to_string())
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut parts = value.split("==");
        let (lhs, rhs) = (
            parts.next().expect("equation should have left-hand side"),
            parts.next().expect("equation should have right-hand side"),
        );

        let lhs = Expression(lhs.split("+").map(Value::from).collect());
        let rhs = Value::from(rhs);

        Equation { lhs, rhs }
    }
}

impl Value {
    fn to_number(&self, candidates: &HashMap<char, u8>) -> Option<u128> {
        // Leading 0s are not valid
        if self.0.starts_with(|c| candidates.get(&c) == Some(&0)) {
            return None;
        }

        let num_str: String = self
            .0
            .chars()
            .map(|c| {
                candidates
                    .get(&c)
                    .expect("letter should be in candidate mapping")
                    .to_string()
            })
            .collect();

        num_str.parse().ok()
    }
}

impl Expression {
    fn sum(&self, candidates: &HashMap<char, u8>) -> Option<u128> {
        self.0
            .iter()
            .map(|value| value.to_number(candidates))
            .try_fold(0_u128, |acc, num| num.map(|n| acc.checked_add(n))?)
    }
}

impl Equation {
    fn is_valid(&self, candidates: &HashMap<char, u8>) -> bool {
        let lhs_sum = self.lhs.sum(candidates);
        let rhs_value = self.rhs.to_number(candidates);

        match (lhs_sum, rhs_value) {
            (Some(lhs), Some(rhs)) => lhs == rhs,
            _ => false,
        }
    }

    fn partial_is_valid(&self, candidates: &HashMap<char, u8>) -> bool {
        if candidates.len() < 3 {
            return true;
        }

        let words: Vec<&str> = self.words();
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0);

        // Tracking carry between columns
        let mut carry = 0;

        // Check if columns are fully assigned
        let mut can_validate = vec![false; max_len];
        for (col, can_validate) in can_validate.iter_mut().enumerate().take(max_len) {
            let column_chars: Vec<char> = words
                .iter()
                .filter_map(|w| {
                    if w.len() > col {
                        let pos = w.len() - 1 - col;
                        w.chars().nth(pos)
                    } else {
                        None
                    }
                })
                .collect();

            *can_validate = column_chars.iter().all(|ch| candidates.contains_key(ch));

            if !*can_validate {
                break;
            }
        }

        // Check columns from least significant to most significant
        for (col, &can_validate) in can_validate.iter().enumerate().take(max_len) {
            // If we can't validate this column, we can't validate any further
            // columns as they would depend on the carry from this column
            if !can_validate {
                break;
            }

            // We start with the carry from the previous column
            let mut sum: u128 = carry;

            // Sum column values for LHS terms
            for w in &self.lhs.0 {
                if w.0.len() > col {
                    let pos = w.0.len() - 1 - col;
                    if let Some(ch) = w.0.chars().nth(pos) {
                        if let Some(&digit) = candidates.get(&ch) {
                            sum += digit as u128;
                        }
                    }
                }
            }

            // Check against RHS value
            if self.rhs.0.len() > col {
                let pos = self.rhs.0.len() - 1 - col;
                if let Some(ch) = self.rhs.0.chars().nth(pos) {
                    if let Some(&digit) = candidates.get(&ch) {
                        if sum % 10 != digit as u128 {
                            return false;
                        }
                    }
                }
            }

            // Update carry for next column
            carry = sum / 10;
        }

        true
    }

    fn ordered_letters(&self) -> Vec<char> {
        let freqs = self.letter_frequencies();
        let first_letters = self.first_letters();
        let mut letters_vec: Vec<char> = self.letters().iter().cloned().collect();

        letters_vec.sort_by(|a, b| {
            let a_is_leading = first_letters.contains(a);
            let b_is_leading = first_letters.contains(b);

            if a_is_leading && !b_is_leading {
                return Ordering::Less;
            }
            if !a_is_leading && b_is_leading {
                return Ordering::Greater;
            }

            let a_freq = freqs.get(a).unwrap_or(&0);
            let b_freq = freqs.get(b).unwrap_or(&0);
            b_freq.cmp(a_freq)
        });
        letters_vec
    }

    fn letters(&self) -> HashSet<char> {
        iter::once(&self.rhs)
            .chain(self.lhs.0.iter())
            .flat_map(|v| v.0.chars())
            .collect()
    }

    fn words(&self) -> Vec<&str> {
        self.lhs
            .0
            .iter()
            .map(|v| v.0.as_str())
            .chain(iter::once(self.rhs.0.as_str()))
            .collect()
    }

    fn first_letters(&self) -> HashSet<char> {
        let lhs = self.lhs.0.iter().flat_map(|v| v.0.chars().take(1));
        self.rhs.0.chars().take(1).chain(lhs).collect()
    }

    fn letter_frequencies(&self) -> HashMap<char, usize> {
        self.lhs
            .0
            .iter()
            .flat_map(|v| v.0.chars())
            .chain(self.rhs.0.chars())
            .fold(HashMap::new(), |mut frequencies, c| {
                *frequencies.entry(c).or_insert(0) += 1;
                frequencies
            })
    }
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::from(input);

    let addend_count = equation.lhs.0.len();

    if addend_count > 30 {
        return solve_with_weights(&equation);
    }

    let first_letters = equation.first_letters();
    let ordered_letters = equation.ordered_letters();
    let mut candidates = HashMap::new();

    backtrack(
        &equation,
        &first_letters,
        &ordered_letters,
        &mut candidates,
        0,
    )
}

fn backtrack(
    equation: &Equation,
    first_letters: &HashSet<char>,
    ordered_letters: &Vec<char>,
    candidates: &mut HashMap<char, u8>,
    index: usize,
) -> Option<HashMap<char, u8>> {
    // Are all letters assigned?
    if index == ordered_letters.len() {
        return if equation.is_valid(candidates) {
            Some(candidates.clone())
        } else {
            None
        };
    }

    let current_letter = ordered_letters[index];
    let used_digits: HashSet<u8> = candidates.values().cloned().collect();

    for digit in 0..10 {
        // Each letter gets its own digit
        if used_digits.contains(&digit) {
            continue;
        }

        // No leading 0s allowed
        if first_letters.contains(&current_letter) && digit == 0 {
            continue;
        }

        // Try a new letter-digit combo
        candidates.insert(current_letter, digit);

        // Before we recurse and lose time, check if the
        // set of candidates is partially valid
        if !equation.partial_is_valid(candidates) {
            candidates.remove(&current_letter);
            continue;
        }

        // Recurse into next letter-digit assignment
        if let Some(solution) = backtrack(
            equation,
            first_letters,
            ordered_letters,
            candidates,
            index + 1,
        ) {
            return Some(solution);
        }

        // Backtrack if this attempt didn't work
        candidates.remove(&current_letter);
    }

    None
}

fn solve_with_weights(equation: &Equation) -> Option<HashMap<char, u8>> {
    // Instead of ordered_letters, we sort by letter weight
    let weights = calculate_letter_weights(equation);
    let mut letters: Vec<char> = weights.keys().cloned().collect();
    letters.sort_by(|a, b| {
        let a_weight = weights.get(a).unwrap_or(&0);
        let b_weight = weights.get(b).unwrap_or(&0);
        b_weight.abs().cmp(&a_weight.abs())
    });

    let first_letters = equation.first_letters();
    let mut candidates = HashMap::new();
    backtrack_with_weights(&weights, &first_letters, &letters, &mut candidates, 0)
}

fn calculate_letter_weights(equation: &Equation) -> HashMap<char, i64> {
    let mut weights = HashMap::new();

    for value in &equation.lhs.0 {
        for (i, c) in value.0.chars().rev().enumerate() {
            let place_value = 10_i64.pow(i as u32);
            *weights.entry(c).or_insert(0) += place_value;
        }
    }

    for (i, c) in equation.rhs.0.chars().rev().enumerate() {
        let place_value = 10_i64.pow(i as u32);
        *weights.entry(c).or_insert(0) -= place_value;
    }

    weights
}

fn backtrack_with_weights(
    weights: &HashMap<char, i64>,
    first_letters: &HashSet<char>,
    letters: &[char],
    candidates: &mut HashMap<char, u8>,
    index: usize,
) -> Option<HashMap<char, u8>> {
    if index == letters.len() {
        // Sum of dot-product
        let sum: i64 = weights
            .iter()
            .map(|(&c, &weight)| {
                let digit = *candidates.get(&c).unwrap() as i64;
                weight * digit
            })
            .sum();

        return if sum == 0 {
            Some(candidates.clone())
        } else {
            None
        };
    }

    let current_letter = letters[index];

    for digit in 0..10 {
        if candidates.values().any(|d| d == &digit) {
            continue;
        }

        if first_letters.contains(&current_letter) && digit == 0 {
            continue;
        }

        candidates.insert(current_letter, digit);

        if let Some(solution) =
            backtrack_with_weights(weights, first_letters, letters, candidates, index + 1)
        {
            return Some(solution);
        }

        candidates.remove(&current_letter);
    }

    None
}
