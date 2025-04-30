use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    // The total number of "moves" it should take to reach the desired number of liters, including
    // the first fill.
    pub moves: u8,
    // Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    // How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Default, Clone, Copy)]
struct State {
    bucket_1: u8,
    bucket_2: u8,
    moves: u8,
}

impl State {
    fn is_goal_reached(&self, goal: u8) -> bool {
        self.bucket_1 == goal || self.bucket_2 == goal
    }

    fn goal_bucket(&self, goal: u8) -> Option<Bucket> {
        if self.bucket_1 == goal {
            Some(Bucket::One)
        } else if self.bucket_2 == goal {
            Some(Bucket::Two)
        } else {
            None
        }
    }

    fn other_bucket(&self, goal: u8) -> Option<u8> {
        match self.goal_bucket(goal) {
            Some(Bucket::One) => Some(self.bucket_2),
            Some(Bucket::Two) => Some(self.bucket_1),
            None => None,
        }
    }

    fn with(&self, bucket_1: Option<u8>, bucket_2: Option<u8>) -> State {
        State {
            bucket_1: bucket_1.unwrap_or(self.bucket_1),
            bucket_2: bucket_2.unwrap_or(self.bucket_2),
            moves: self.moves + 1,
        }
    }

    fn next_states(&self, capacity_1: u8, capacity_2: u8, start_bucket: &Bucket) -> Vec<State> {
        let mut states = Vec::new();

        // Fill actions
        if self.bucket_1 < capacity_1 {
            states.push(self.with(Some(capacity_1), None))
        }
        if self.bucket_2 < capacity_2 {
            states.push(self.with(None, Some(capacity_2)))
        }

        // Empty actions
        if self.bucket_1 > 0 {
            states.push(self.with(Some(0), None))
        }
        if self.bucket_2 > 0 {
            states.push(self.with(None, Some(0)))
        }

        // Pour actions
        if self.bucket_1 > 0 && self.bucket_2 < capacity_2 {
            let pour = self.bucket_1.min(capacity_2 - self.bucket_2);
            states.push(self.with(Some(self.bucket_1 - pour), Some(self.bucket_2 + pour)))
        }
        if self.bucket_2 > 0 && self.bucket_1 < capacity_1 {
            let pour = self.bucket_2.min(capacity_1 - self.bucket_1);
            states.push(self.with(Some(self.bucket_1 + pour), Some(self.bucket_2 - pour)))
        }

        // After an action, we may not arrive at a state where the initial
        // starting bucket is empty and the other bucket is full.
        states
            .into_iter()
            .filter(|s| match start_bucket {
                Bucket::One => !(s.bucket_1 == 0 && s.bucket_2 == capacity_2),
                Bucket::Two => !(s.bucket_2 == 0 && s.bucket_1 == capacity_1),
            })
            .collect()
    }
}

// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    if !is_problem_solvable(capacity_1, capacity_2, goal) {
        return None;
    }

    let mut initial_state = State::default();
    // Fill the starting bucket
    match start_bucket {
        Bucket::One => initial_state.bucket_1 = capacity_1,
        Bucket::Two => initial_state.bucket_2 = capacity_2,
    }
    initial_state.moves += 1;

    // Keep a queue for tracking states
    let max_states = ((capacity_1 as usize) + 1) * ((capacity_2 as usize) + 1);
    let mut queue = VecDeque::with_capacity(max_states);
    queue.push_back(initial_state);

    // Keep a set to avoid cycles
    let mut visited = HashSet::with_capacity(max_states);
    visited.insert((initial_state.bucket_1, initial_state.bucket_2));

    while let Some(current) = queue.pop_front() {
        if current.is_goal_reached(goal) {
            return Some(BucketStats {
                moves: current.moves,
                goal_bucket: current.goal_bucket(goal).expect("goal should be reached!"),
                other_bucket: current.other_bucket(goal).expect("goal should be reached!"),
            });
        }

        // Generate next valid states and insert them into the queue
        for next_state in current.next_states(capacity_1, capacity_2, start_bucket) {
            let state_key = (next_state.bucket_1, next_state.bucket_2);
            if !visited.contains(&state_key) {
                visited.insert(state_key);
                queue.push_back(next_state);
            }
        }
    }

    None
}

fn is_problem_solvable(capacity_1: u8, capacity_2: u8, goal: u8) -> bool {
    goal <= capacity_1.max(capacity_2) && goal % gcd(capacity_1, capacity_2) == 0
}

fn gcd(a: u8, b: u8) -> u8 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
