#![allow(unused_variables)]

use nom::character::complete::{i32, line_ending};
use nom::multi::separated_list0;
use nom::IResult;
use std::ops::{Index, IndexMut};
use std::time::{Duration, Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub trait Problem<'a> {
    fn new(input: &'a str) -> Self;
    fn solve_part_one(&self) -> usize;
    fn solve_part_two(&self) -> usize;
}

fn parse_number_list(s: &str) -> Vec<i32> {
    let res: IResult<_, _, nom::error::Error<&str>> = separated_list0(line_ending, i32)(s);
    res.unwrap().1
}

#[derive(Clone, PartialEq)]
struct Bitmap<T: Clone> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Clone> Bitmap<T> {
    fn new(init: T, width: usize, height: usize) -> Bitmap<T> {
        Bitmap {
            data: vec![init; width * height],
            width,
            height,
        }
    }

    fn as_slice(&self) -> &[T] {
        self.data.as_slice()
    }

    fn is_idx_in_range(&self, x: isize, y: isize) -> bool {
        !(x < 0 || (x as usize) >= self.width || y < 0 || (y as usize) >= self.height)
    }

    fn get(&self, x: isize, y: isize) -> Option<&T> {
        if !self.is_idx_in_range(x, y) {
            return None;
        }

        Some(&self.data[(y as usize) * self.width + x as usize])
    }

    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut T> {
        if !self.is_idx_in_range(x, y) {
            return None;
        }

        Some(&mut self.data[(y as usize) * self.width + x as usize])
    }

    fn get_side(&self, side: u32) -> Vec<T> {
        todo!()
    }
}

impl<T: Clone> Index<(isize, isize)> for Bitmap<T> {
    type Output = T;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let (x, y) = index;
        if !self.is_idx_in_range(x, y) {
            panic!();
        }

        &self.data[y as usize * self.width + x as usize]
    }
}

impl<T: Clone> IndexMut<(isize, isize)> for Bitmap<T> {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let (x, y) = index;
        if !self.is_idx_in_range(x, y) {
            panic!();
        }

        &mut self.data[y as usize * self.width + x as usize]
    }
}

fn main() {
    let input = include_str!("../example.txt");

    let instance = day23::Instance::new(input);

    println!("{}", instance.solve_part_one());

    let start_part_two = Instant::now();
    println!("{}", instance.solve_part_two());
    let end_part_two = Instant::now();

    println!(
        "Part two: {}ms",
        end_part_two.duration_since(start_part_two).as_micros() as f64 / 1000.0
    );
}
