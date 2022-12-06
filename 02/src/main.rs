use common::read_input;

const OPP_ROCK: char = 'A';
const OPP_PAPER: char = 'B';
const OPP_SCISSOR: char = 'C';
const ROCK: char = 'X';
const PAPER: char = 'Y';
const SCISSOR: char = 'Z';
const LOSE: char = 'X';
const DRAW: char = 'Y';
const WIN: char = 'Z';

fn main() {
    let path = "input.txt";
    let lines = read_input(path).expect("unreadable");

    let mut score = 0;
    for line in lines {
        if line.len() == 0 {
            continue;
        }
        let (opp, resp) = {
            let chars: Vec<char> = line.chars().collect();
            (chars[0], chars[2])
        };
        score += get_score(opp, resp);
    }

    println!("score: {}", score);
}

fn get_score(opp: char, outcome: char) -> usize {
    let resp = get_resp_from_outcome(opp, outcome);
    get_move_score(resp) + get_h2h_score(opp, resp)
}

fn get_resp_from_outcome(opp: char, outcome: char) -> char {
    match (opp, outcome) {
        (OPP_ROCK, WIN) => PAPER,
        (OPP_ROCK, LOSE) => SCISSOR,
        (OPP_ROCK, DRAW) => ROCK,
        (OPP_PAPER, WIN) => SCISSOR,
        (OPP_PAPER, LOSE) => ROCK,
        (OPP_PAPER, DRAW) => PAPER,
        (OPP_SCISSOR, WIN) => ROCK,
        (OPP_SCISSOR, LOSE) => PAPER,
        (OPP_SCISSOR, DRAW) => SCISSOR,
        _ => unreachable!(),
    }
}

fn get_move_score(resp: char) -> usize {
    match resp {
        ROCK => 1,
        PAPER => 2,
        SCISSOR => 3,
        _ => unreachable!(),
    }
}

fn get_h2h_score(opp: char, resp: char) -> usize {
    match (opp, resp) {
        (OPP_ROCK, ROCK) => 3,
        (OPP_ROCK, PAPER) => 6,
        (OPP_ROCK, SCISSOR) => 0,
        (OPP_PAPER, ROCK) => 0,
        (OPP_PAPER, PAPER) => 3,
        (OPP_PAPER, SCISSOR) => 6,
        (OPP_SCISSOR, ROCK) => 6,
        (OPP_SCISSOR, PAPER) => 0,
        (OPP_SCISSOR, SCISSOR) => 3,
        _ => unreachable!(),
    }
}
