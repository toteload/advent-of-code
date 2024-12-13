use std::collections::HashMap;

use crate::{Bitmap, Problem};

pub struct Instance {
    seats: Vec<(isize, isize)>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
fn print_seating_arrangement(inst: &Instance, occupied: &[bool]) {
    let mut seats_bitmap = vec![false; inst.width * inst.height];

    for (x, y) in &inst.seats {
        seats_bitmap[(*y as usize) * inst.width + *x as usize] = true;
    }

    for y in 0..inst.height {
        for x in 0..inst.width {
            let idx = y * inst.width + x;

            if occupied[idx] {
                print!("#");
            } else if seats_bitmap[idx] {
                print!("L");
            } else {
                print!(".");
            }
        }

        println!();
    }
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let lines = input.lines().collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let mut seats = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                if *c == b'L' {
                    seats.push((x as isize, y as isize));
                }
            }
        }

        Instance {
            seats,
            width,
            height,
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut occupied = Bitmap::new(false, self.width, self.height);

        loop {
            let mut next = occupied.clone();

            for &(sx, sy) in &self.seats {
                let neighbours = [
                    (-1, -1),
                    (0, -1),
                    (1, -1),
                    (-1, 0),
                    (1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                ];

                let mut neighbour_count = 0;
                for (dx, dy) in &neighbours {
                    if let Some(v) = occupied.get(sx + dx, sy + dy) {
                        if *v {
                            neighbour_count += 1;
                        }
                    }
                }

                if !occupied[(sx, sy)] && neighbour_count == 0 {
                    next[(sx, sy)] = true;
                }

                if occupied[(sx, sy)] && neighbour_count >= 4 {
                    next[(sx, sy)] = false;
                }
            }

            if next == occupied {
                break;
            }

            occupied = next;
        }

        self.seats
            .iter()
            .filter_map(|&(x, y)| if occupied[(x, y)] { Some(()) } else { None })
            .count()
    }

    fn solve_part_two(&self) -> usize {
        let mut seat_bitmap = Bitmap::new(false, self.width, self.height);

        for &(x, y) in &self.seats {
            seat_bitmap[(x, y)] = true;
        }

        const DIRECTIONS: [(isize, isize); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut visibility_map = HashMap::new();

        for &(sx, sy) in &self.seats {
            let mut visible_seats = Vec::new();
            for &(dx, dy) in &DIRECTIONS {
                let mut i = 1;
                let (mut x, mut y) = (sx + dx, sy + dy);
                while let Some(is_seat) = seat_bitmap.get(x, y) {
                    if *is_seat {
                        visible_seats.push((x, y));
                        break;
                    }

                    i += 1;
                    (x, y) = (sx + i * dx, sy + i * dy);
                }
            }

            visibility_map.insert((sx, sy), visible_seats);
        }

        let mut occupied = Bitmap::new(false, self.width, self.height);

        loop {
            let mut next = occupied.clone();

            for seat @ &(sx, sy) in &self.seats {
                let mut neighbour_count = 0;
                for &(x, y) in visibility_map.get(seat).unwrap().iter() {
                    if let Some(v) = occupied.get(x, y) {
                        if *v {
                            neighbour_count += 1;
                        }
                    }
                }

                if !occupied[(sx, sy)] && neighbour_count == 0 {
                    next[(sx, sy)] = true;
                }

                if occupied[(sx, sy)] && neighbour_count >= 5 {
                    next[(sx, sy)] = false;
                }
            }

            if next == occupied {
                break;
            }

            occupied = next;
        }

        self.seats
            .iter()
            .filter_map(|&(x, y)| if occupied[(x, y)] { Some(()) } else { None })
            .count()
    }
}
