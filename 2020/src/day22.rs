use std::collections::{HashSet, VecDeque};

use nom::{multi::separated_list0, sequence::terminated};

use crate::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, u32},
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_player_deck(s: &str) -> IResult<&str, Vec<u32>> {
    let (s, _) = preceded(tag("Player "), u32)(s)?;
    let (s, _) = terminated(char(':'), line_ending)(s)?;
    let (s, deck) = separated_list0(line_ending, u32)(s)?;
    let (s, _) = line_ending(s)?;
    Ok((s, deck))
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list0(line_ending, parse_player_deck)(s)
}

pub struct Instance {
    decks: [Vec<u32>; 2],
}

#[derive(Copy, Clone)]
enum Outcome {
    Instawin,
    Regular,
}

fn recursive_combat(cards: [&[u32]; 2]) -> (Outcome, [VecDeque<u32>; 2]) {
    let mut history = HashSet::new();

    let mut decks = cards.map(|d| d.iter().copied().collect::<VecDeque<_>>());
    while !decks[0].is_empty() && !decks[1].is_empty() {
        if history.contains(&decks) {
            return (Outcome::Instawin, decks);
        }

        history.insert(decks.clone());

        if let (Some(a), Some(b)) = (decks[0].pop_front(), decks[1].pop_front()) {
            let winner = if (a as usize) <= decks[0].len() && (b as usize) <= decks[1].len() {
                decks[0].make_contiguous();
                decks[1].make_contiguous();

                let (outcome, subdecks) = recursive_combat([
                    &decks[0].as_slices().0[..(a as usize)],
                    &decks[1].as_slices().0[..(b as usize)],
                ]);

                match outcome {
                    Outcome::Regular if subdecks[0].is_empty() => 1,
                    _ => 0,
                }
            } else if a > b {
                0
            } else {
                1
            };

            if winner == 0 {
                decks[0].push_back(a);
                decks[0].push_back(b);
            } else {
                decks[1].push_back(b);
                decks[1].push_back(a);
            }
        }
    }

    (Outcome::Regular, decks)
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let decks = parse(input).unwrap().1;
        Instance {
            decks: [decks[0].clone(), decks[1].clone()],
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut decks = self
            .decks
            .iter()
            .map(|d| d.iter().copied().collect::<VecDeque<_>>())
            .collect::<Vec<_>>();

        while !decks[0].is_empty() && !decks[1].is_empty() {
            if let (Some(a), Some(b)) = (decks[0].pop_front(), decks[1].pop_front()) {
                if a > b {
                    decks[0].push_back(a);
                    decks[0].push_back(b);
                } else {
                    decks[1].push_back(b);
                    decks[1].push_back(a);
                }
            }
        }

        let winner = if decks[0].is_empty() {
            &decks[1]
        } else {
            &decks[0]
        };

        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * (*x as usize))
            .sum()
    }

    fn solve_part_two(&self) -> usize {
        let deck_slices = [self.decks[0].as_slice(), self.decks[1].as_slice()];
        let (outcome, decks) = recursive_combat(deck_slices);
        let winner = match outcome {
            Outcome::Regular if decks[0].is_empty() => &decks[1],
            _ => &decks[0],
        };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(i, x)| (i + 1) * (*x as usize))
            .sum()
    }
}
