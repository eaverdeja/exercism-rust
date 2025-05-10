use std::collections::{HashMap, HashSet};

use Category::*;

#[derive(Debug, Clone, Copy)]
pub enum Category {
    Ones = 1,
    Twos = 2,
    Threes = 3,
    Fours = 4,
    Fives = 5,
    Sixes = 6,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Yacht => {
            if HashSet::from(dice).len() == 1 {
                50
            } else {
                0
            }
        }
        Choice => dice.iter().sum(),
        BigStraight => {
            let mut dice = dice;
            dice.sort();
            match dice {
                [2, 3, 4, 5, 6] => 30,
                _ => 0,
            }
        }
        LittleStraight => {
            let mut dice = dice;
            dice.sort();
            match dice {
                [1, 2, 3, 4, 5] => 30,
                _ => 0,
            }
        }
        FourOfAKind => {
            let tally = build_tally(&dice);

            match tally.iter().find(|&(_, &count)| count >= 4) {
                Some((&die, _)) => die * 4,
                None => 0,
            }
        }
        FullHouse => {
            let tally = build_tally(&dice);

            let has_two = tally.values().any(|&count| count == 2);
            let has_three = tally.values().any(|&count| count == 3);

            if has_two && has_three {
                dice.iter().sum()
            } else {
                0
            }
        }
        Ones | Twos | Threes | Fours | Fives | Sixes => {
            dice.iter().filter(|d| **d == category as u8).sum()
        }
    }
}

fn build_tally(dice: &Dice) -> HashMap<&u8, u8> {
    dice.iter().fold(HashMap::new(), |mut tally, die| {
        tally.entry(die).and_modify(|d| *d += 1).or_insert(1);
        tally
    })
}
