use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    character::complete::{char, line_ending},
    multi::{many1, many_till},
    sequence::preceded,
    IResult, Parser,
};

use crate::Problem;

enum HexDirection {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

type TileIndex = (i32, i32, i32);

fn tile_neighbours(t: &TileIndex) -> [TileIndex; 6] {
    let (x, y, z) = *t;
    [
        (x, y + 1, z + 1),
        (x - 1, y, z + 1),
        (x - 1, y - 1, z),
        (x, y - 1, z - 1),
        (x + 1, y, z - 1),
        (x + 1, y + 1, z),
    ]
}

pub struct Instance {
    tiles: Vec<Vec<HexDirection>>,
}

fn parse_hex_direction(s: &str) -> IResult<&str, HexDirection> {
    use HexDirection::*;
    alt((
        preceded(
            char('n'),
            alt((char('e').map(|_| NorthEast), char('w').map(|_| NorthWest))),
        ),
        preceded(
            char('s'),
            alt((char('e').map(|_| SouthEast), char('w').map(|_| SouthWest))),
        ),
        char('e').map(|_| East),
        char('w').map(|_| West),
    ))(s)
}

fn parse(s: &str) -> IResult<&str, Vec<Vec<HexDirection>>> {
    many1(many_till(parse_hex_direction, line_ending).map(|(o, _)| o))(s)
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        Instance {
            tiles: parse(input).unwrap().1,
        }
    }

    fn solve_part_one(&self) -> usize {
        let tiles = self
            .tiles
            .iter()
            .map(|dirs| {
                dirs.iter().fold((0, 0, 0), |(x, y, z), dir| {
                    use HexDirection::*;
                    match dir {
                        NorthEast => (x, y + 1, z + 1),
                        East => (x - 1, y, z + 1),
                        SouthEast => (x - 1, y - 1, z),
                        SouthWest => (x, y - 1, z - 1),
                        West => (x + 1, y, z - 1),
                        NorthWest => (x + 1, y + 1, z),
                    }
                })
            })
            .collect::<Vec<_>>();

        let mut flips = HashMap::new();
        for tile in tiles {
            *flips.entry(tile).or_insert(0) += 1;
        }

        flips.values().filter(|x| *x % 2 == 1).count()
    }

    fn solve_part_two(&self) -> usize {
        let init_black = {
            let tiles = self
                .tiles
                .iter()
                .map(|dirs| {
                    dirs.iter().fold((0, 0, 0), |(x, y, z), dir| {
                        use HexDirection::*;
                        match dir {
                            NorthEast => (x, y + 1, z + 1),
                            East => (x - 1, y, z + 1),
                            SouthEast => (x - 1, y - 1, z),
                            SouthWest => (x, y - 1, z - 1),
                            West => (x + 1, y, z - 1),
                            NorthWest => (x + 1, y + 1, z),
                        }
                    })
                })
                .collect::<Vec<_>>();

            let mut flips = HashMap::new();
            for tile in tiles {
                *flips.entry(tile).or_insert(0) += 1;
            }

            flips
                .iter()
                .filter_map(|(i, x)| if *x % 2 == 1 { Some(*i) } else { None })
                .collect::<Vec<_>>()
        };

        let mut black = init_black.into_iter().collect::<HashSet<_>>();

        for _ in 0..100 {
            let mut next = black.clone();

            for tile in black.iter() {
                let black_neighbour_count = tile_neighbours(tile)
                    .iter()
                    .filter(|x| black.contains(*x))
                    .count();
                if black_neighbour_count == 0 || black_neighbour_count > 2 {
                    next.remove(tile);
                }
            }

            for tile in black.iter() {
                for t in tile_neighbours(tile) {
                    if black.contains(&t) {
                        continue;
                    }

                    let white_tile = t;
                    let black_neighbour_count = tile_neighbours(&white_tile)
                        .iter()
                        .filter(|x| black.contains(*x))
                        .count();
                    if black_neighbour_count == 2 {
                        next.insert(white_tile);
                    }
                }
            }

            black = next;
        }

        black.len()
    }
}
