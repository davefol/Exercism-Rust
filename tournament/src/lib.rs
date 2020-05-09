use std::collections::HashMap;
use MatchResult::*;

enum MatchResult {
    Win,
    Loss,
    Draw,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Team<'a> {
    points: u32,
    name: &'a str,
    wins: u32,
    draws: u32,
    losses: u32,
}

impl<'a> Team<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            points: 0,
            name: name,
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn matches_played(&self) -> u32 {
        self.wins + self.draws + self.losses
    }

    fn calculate_points(&self) -> u32 {
        (3 * self.wins) + (self.draws)
    }

    fn update(&mut self, result: MatchResult) {
        match result {
            Win => self.wins += 1,
            Loss => self.losses += 1,
            Draw => self.draws += 1,
        }
        self.points = self.calculate_points();
    }

    fn display(&self) -> String {
        format!(
            "{:<31}|{:>3} |{:>3} |{:>3} |{:>3} |{:>3}",
            self.name,
            self.matches_played(),
            self.wins,
            self.draws,
            self.losses,
            self.points
        )
    }
}

pub fn tally(match_results: &str) -> String {
    let mut teams = HashMap::<&str, Team>::new();
    for match_row in match_results.lines() {
        teams = parse_row(match_row, teams);
    }
    // TODO: compartor function

    let mut table = vec![String::from("Team                           | MP |  W |  D |  L |  P")];
    let mut sorted_teams = teams.values().collect::<Vec<&Team>>();
    sorted_teams.sort_by(|a, b| a.points.cmp(&b.points).reverse().then(a.name.cmp(b.name)));
    let mut rest_of_table = sorted_teams
            .iter()
            .map(|x| x.display())
            .collect::<Vec<String>>();
    table.append(
        &mut rest_of_table
    );
    table.join("\n")
}

fn parse_row<'a>(
    match_row: &'a str,
    mut t: HashMap<&'a str, Team<'a>>,
) -> HashMap<&'a str, Team<'a>> {
    let (first_team, second_team, (first_team_result, second_team_result)) =
        tokenize_row(match_row);
    t.entry(first_team)
        .or_insert(Team::new(first_team))
        .update(first_team_result);
    t.entry(second_team)
        .or_insert(Team::new(second_team))
        .update(second_team_result);
    t
}

fn tokenize_row(match_row: &str) -> (&str, &str, (MatchResult, MatchResult)) {
    let m: Vec<&str> = match_row.split(';').collect();
    (
        m[0],
        m[1],
        match m[2] {
            "win" => (Win, Loss),
            "draw" => (Draw, Draw),
            "loss" => (Loss, Win),
            _ => panic!(),
        },
    )
}
