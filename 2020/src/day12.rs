use nom::{
    character::complete::{i32, line_ending, one_of},
    multi::separated_list0,
    IResult,
};

use crate::Problem;

pub struct Instance {
    actions: Vec<Action>,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        use Direction::*;
        match *self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    fn turn_right(&self) -> Direction {
        use Direction::*;
        match *self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }
}

struct Ship {
    facing: Direction,
    x: i32,
    y: i32,
}

impl Ship {
    fn move_in_direction(&mut self, dir: Direction, z: i32) {
        use Direction::*;
        match dir {
            North => self.y += z,
            South => self.y -= z,
            East => self.x += z,
            West => self.x -= z,
        }
    }
}

enum Action {
    Move(Direction, i32),
    Forward(i32),
    TurnLeft(i32),
    TurnRight(i32),
}

fn parse_action(s: &str) -> IResult<&str, Action> {
    let (s, action_char) = (one_of("NSEWLRF"))(s)?;
    let (s, z) = i32(s)?;

    use Action::*;
    use Direction::*;

    let action = match action_char {
        'N' => Move(North, z),
        'S' => Move(South, z),
        'E' => Move(East, z),
        'W' => Move(West, z),
        'F' => Forward(z),
        'L' => TurnLeft(z / 90),
        'R' => TurnRight(z / 90),
        _ => unreachable!(),
    };

    Ok((s, action))
}

fn parse(input: &str) -> Vec<Action> {
    separated_list0(line_ending, parse_action)(input).unwrap().1
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        Instance {
            actions: parse(input),
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut ship = Ship {
            facing: Direction::East,
            x: 0,
            y: 0,
        };

        for action in &self.actions {
            use Action::*;

            match *action {
                TurnLeft(n) => {
                    for _ in 0..n {
                        ship.facing = ship.facing.turn_left()
                    }
                }
                TurnRight(n) => {
                    for _ in 0..n {
                        ship.facing = ship.facing.turn_right()
                    }
                }
                Move(dir, z) => ship.move_in_direction(dir, z),
                Forward(z) => ship.move_in_direction(ship.facing, z),
            }
        }

        (ship.x.unsigned_abs() + ship.y.unsigned_abs()) as usize
    }

    fn solve_part_two(&self) -> usize {
        let mut waypoint = (10i32, 1);
        let mut ship = Ship {
            facing: Direction::East,
            x: 0,
            y: 0,
        };

        for action in &self.actions {
            use Action::*;

            match *action {
                TurnLeft(n) => {
                    for _ in 0..n {
                        waypoint = (-waypoint.1, waypoint.0);
                    }
                }
                TurnRight(n) => {
                    for _ in 0..n {
                        waypoint = (waypoint.1, -waypoint.0);
                    }
                }
                Move(dir, z) => {
                    use Direction::*;
                    match dir {
                        North => waypoint.1 += z,
                        South => waypoint.1 -= z,
                        East => waypoint.0 += z,
                        West => waypoint.0 -= z,
                    }
                }
                Forward(z) => {
                    ship.x += z * waypoint.0;
                    ship.y += z * waypoint.1;
                }
            }
        }

        (ship.x.unsigned_abs() + ship.y.unsigned_abs()) as usize
    }
}
