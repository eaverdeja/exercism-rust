use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";
const WIN: &str = "win";
const DRAW: &str = "draw";
const LOSS: &str = "loss";

#[derive(Debug, Default)]
struct TeamStats {
    wins: u8,
    losses: u8,
    draws: u8,
}

#[derive(Debug)]
struct Tally<'a>(HashMap<&'a str, TeamStats>);

impl Tally<'_> {
    fn new() -> Self {
        Tally(HashMap::new())
    }

    fn matches_played(&self, team: &str) -> u8 {
        self.0
            .get(team)
            .map_or(0, |stats| stats.wins + stats.losses + stats.draws)
    }

    fn points(&self, team: &str) -> u8 {
        self.0
            .get(team)
            .map_or(0, |stats| stats.wins * 3 + stats.draws)
    }

    fn wins(&self, team: &str) -> u8 {
        self.0.get(team).map_or(0, |stats| stats.wins)
    }

    fn draws(&self, team: &str) -> u8 {
        self.0.get(team).map_or(0, |stats| stats.draws)
    }

    fn losses(&self, team: &str) -> u8 {
        self.0.get(team).map_or(0, |stats| stats.losses)
    }

    fn teams(&self) -> Vec<&str> {
        let mut teams: Vec<&str> = self.keys().cloned().collect();
        teams.sort_by_cached_key(|team| (-i32::from(self.points(team)), *team));
        teams
    }
}

impl<'a> Deref for Tally<'a> {
    type Target = HashMap<&'a str, TeamStats>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Tally<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn tally(match_results: &str) -> String {
    let result = HEADER.to_string();
    if match_results.is_empty() {
        return result;
    }

    let tally = build_tally(match_results);
    build_result(result, tally)
}

fn build_tally(match_results: &str) -> Tally {
    let mut tally = Tally::new();

    for match_result in match_results.split("\n") {
        let mut parts = match_result.split(";");
        let (team_a, team_b, outcome) = (
            parts.next().expect("Team A should be present"),
            parts.next().expect("Team B should be present"),
            parts.next().expect("Outcome should be present"),
        );

        match outcome {
            WIN => {
                process_match(&mut tally, team_a, WIN);
                process_match(&mut tally, team_b, LOSS);
            }
            LOSS => {
                process_match(&mut tally, team_a, LOSS);
                process_match(&mut tally, team_b, WIN);
            }
            _ => {
                process_match(&mut tally, team_a, DRAW);
                process_match(&mut tally, team_b, DRAW);
            }
        }
    }

    tally
}

fn process_match<'a>(tally: &mut Tally<'a>, team: &'a str, outcome: &str) {
    tally
        .entry(team)
        .and_modify(|team_tally| process_outcome(team_tally, outcome))
        .or_insert_with(|| {
            let mut team_tally = TeamStats::default();
            process_outcome(&mut team_tally, outcome);
            team_tally
        });
}

fn process_outcome(team_tally: &mut TeamStats, outcome: &str) {
    match outcome {
        WIN => team_tally.wins += 1,
        LOSS => team_tally.losses += 1,
        DRAW => team_tally.draws += 1,
        _ => {}
    };
}

fn build_result(mut result: String, tally: Tally) -> String {
    let divider_idx = result.find('|').expect("| should be present");
    let (title, _) = result.split_at(divider_idx);
    let title_length = title.len();

    for team in tally.teams() {
        result = format!(
            "{result}\n{:<width$}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team,
            tally.matches_played(team),
            tally.wins(team),
            tally.draws(team),
            tally.losses(team),
            tally.points(team),
            width = title_length
        )
    }

    result
}
