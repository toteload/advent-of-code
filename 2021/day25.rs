use std::io::{self, BufRead};
use std::fmt;

#[derive(Debug)]
struct State {
    width: usize,
    height: usize,
    east_cucumbers: Vec<bool>,
    south_cucumbers: Vec<bool>,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.east_cucumbers[idx] {
                    write!(f, ">")?;
                } else if self.south_cucumbers[idx] {
                    write!(f, "v")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl State {
    fn new_from_input_lines(input: &[String]) -> State {
        let width = input[0].len();
        let height = input.len();

        let mut east_cucumbers = Vec::new();
        east_cucumbers.resize(width*height, false);

        let mut south_cucumbers = Vec::new();
        south_cucumbers.resize(width*height, false);

        for y in 0..height {
            let bytes = input[y].as_bytes();
            for x in 0..width {
                let idx = y * width + x;

                if bytes[x] == 'v' as u8 {
                    south_cucumbers[idx] = true;
                }

                if bytes[x] == '>' as u8 {
                    east_cucumbers[idx] = true;
                }
            }
        }

        State {
            width,
            height,
            east_cucumbers,
            south_cucumbers,
        }
    }

    fn update(&mut self) {
        let mut next_east_cucumbers = vec![false; self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                if !self.east_cucumbers[idx] {
                    continue;
                }

                let next_idx = y * self.width + (x + 1) % self.width;

                if !(self.east_cucumbers[next_idx] || self.south_cucumbers[next_idx]) {
                    next_east_cucumbers[next_idx] = true;
                } else {
                    next_east_cucumbers[idx] = true;
                }
            }
        }

        self.east_cucumbers = next_east_cucumbers;

        let mut next_south_cucumbers = vec![false; self.width * self.height];

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                if !self.south_cucumbers[idx] {
                    continue;
                }

                let next_idx = ((y + 1) % self.height) * self.width + x;

                if !(self.east_cucumbers[next_idx] || self.south_cucumbers[next_idx]) {
                    next_south_cucumbers[next_idx] = true;
                } else {
                    next_south_cucumbers[idx] = true;
                }
            }
        }

        self.south_cucumbers = next_south_cucumbers;
    }

    fn can_move(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                if !self.east_cucumbers[idx] {
                    continue;
                }

                let next_idx = y * self.width + (x + 1) % self.width;

                if !(self.east_cucumbers[next_idx] || self.south_cucumbers[next_idx]) {
                    return true;
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;

                if !self.south_cucumbers[idx] {
                    continue;
                }

                let next_idx = ((y + 1) % self.height) * self.width + x;

                if !(self.east_cucumbers[next_idx] || self.south_cucumbers[next_idx]) {
                    return true;
                }
            }
        }

        false
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut cucumbers = State::new_from_input_lines(&lines);

    let mut counter = 1;
    while cucumbers.can_move() {
        cucumbers.update();
        counter += 1;
    }

    println!("{}", counter);
}
