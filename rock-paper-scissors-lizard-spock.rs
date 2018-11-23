use std::collections::HashMap;

static ROCK: &'static str = "ROCK";
static PAPER: &'static str = "PAPER";
static SCISSORS: &'static str = "SCISSORS";
static LIZARD: &'static str = "LIZARD";
static SPOCK: &'static str = "SPOCK";

fn rock_paper_scissors_lizard_spock(player1: &str, player2: &str) -> String {
    let mut character_adversary_map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
    character_adversary_map.insert(ROCK, vec![PAPER, SPOCK]);
    character_adversary_map.insert(PAPER, vec![SCISSORS, SPOCK]);
    character_adversary_map.insert(SCISSORS, vec![ROCK, SPOCK]);
    character_adversary_map.insert(LIZARD, vec![SCISSORS, ROCK]);
    character_adversary_map.insert(SPOCK, vec![LIZARD, PAPER]);

    if character_adversary_map
        .get(player1)
        .unwrap()
        .contains(&player2)
    {
        player2.to_string()
    } else {
        player1.to_string()
    }
}

fn main() {
    let player1 = PAPER;
    let player2 = SPOCK;
    println!(
        "{} Wins!",
        rock_paper_scissors_lizard_spock(player1, player2)
    );
}