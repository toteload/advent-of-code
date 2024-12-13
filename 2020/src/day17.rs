use crate::{Bitmap, Problem};
use itertools::{iproduct, Itertools, MinMaxResult};
use std::collections::HashSet;

pub struct Instance {
    init_cubes: Vec<bool>,
    width: usize,
    height: usize,
}

type Position = (i32, i32, i32, i32);

fn print_cubes(cubes: &HashSet<Position>) {
    //use MinMaxResult::*;

    //let MinMax(&minx, &maxx) = cubes.iter().map(|(x,_,_)| x).minmax() else { panic!() };
    //let MinMax(&miny, &maxy) = cubes.iter().map(|(_,y,_)| y).minmax() else { panic!() };
    //let MinMax(&minz, &maxz) = cubes.iter().map(|(_,_,z)| z).minmax() else { panic!() };

    //println!("x:{}..{}, y:{}..{}, z:{}..{}", minx,maxx,miny,maxy,minz,maxz);

    //for z in minz..=maxz {
    //    println!("z={}", z);

    //    for y in miny..=maxy {
    //        for x in minx..=maxx {
    //            if cubes.contains(&(x,y,z)) {
    //                print!("#");
    //            } else {
    //                print!(".");
    //            }
    //        }

    //        println!();
    //    }

    //    println!();
    //}
}

fn neighbours(p: &Position) -> [Position; 80] {
    let (x, y, z, w) = *p;

    let mut res = [(0, 0, 0, 0); 80];
    let mut i = 0;

    for (dx, dy, dz, dw) in iproduct!(-1..=1, -1..=1, -1..=1, -1..=1) {
        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
            continue;
        }

        res[i] = (x + dx, y + dy, z + dz, w + dw);
        i += 1;
    }

    res
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        let lines = input.lines().collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let bitmap = lines
            .iter()
            .flat_map(|line| line.as_bytes().iter().map(|c| *c == b'#'))
            .collect();

        Instance {
            init_cubes: bitmap,
            width,
            height,
        }
    }

    fn solve_part_one(&self) -> usize {
        // This code doesn't work any more due to changes made to make part two work.
        // I didn't want to split up this day into 2 files.

        //let mut cubes = HashSet::<Position>::new();

        //for (x, y) in iproduct!(0..self.width, 0..self.height) {
        //    if self.init_cubes[y * self.width + x] {
        //        cubes.insert((x as i32, y as i32, 0, 0));
        //    }
        //}

        //for _ in 0..6 {
        //    let mut next = cubes.clone();

        //    for cube in cubes.iter() {
        //        let ns = neighbours(cube);

        //        let active_neighbour_count = ns.iter().filter(|c| cubes.contains(c)).count();

        //        if !(active_neighbour_count >= 2 && active_neighbour_count <= 3) {
        //            next.remove(cube);
        //        }

        //        for n in &ns {
        //            if cubes.contains(n) || next.contains(n) {
        //                continue;
        //            }

        //            // This is an inactive cube that is neighbouring at least one active cube.

        //            let ms = neighbours(n);
        //            let active_neighbour_count = ms.iter().filter(|c| cubes.contains(c)).count();

        //            if active_neighbour_count == 3 {
        //                next.insert(*n);
        //            }
        //        }
        //    }

        //    cubes = next;
        //}

        //cubes.len()

        0
    }

    fn solve_part_two(&self) -> usize {
        let mut cubes = HashSet::<Position>::new();

        for (x, y) in iproduct!(0..self.width, 0..self.height) {
            if self.init_cubes[y * self.width + x] {
                cubes.insert((x as i32, y as i32, 0, 0));
            }
        }

        for _ in 0..6 {
            let mut next = cubes.clone();

            for cube in cubes.iter() {
                let ns = neighbours(cube);

                let active_neighbour_count = ns.iter().filter(|c| cubes.contains(c)).count();

                if !(active_neighbour_count >= 2 && active_neighbour_count <= 3) {
                    next.remove(cube);
                }

                for n in &ns {
                    if cubes.contains(n) || next.contains(n) {
                        continue;
                    }

                    // This is an inactive cube that is neighbouring at least one active cube.

                    let ms = neighbours(n);
                    let active_neighbour_count = ms.iter().filter(|c| cubes.contains(c)).count();

                    if active_neighbour_count == 3 {
                        next.insert(*n);
                    }
                }
            }

            cubes = next;
        }

        cubes.len()
    }
}
