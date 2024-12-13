use crate::Problem;
use core::slice;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, line_ending, u32};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::{IResult, Parser};
use std::collections::{HashMap, HashSet};

pub struct Instance<'a> {
    bags: HashMap<&'a str, Vec<(u32, &'a str)>>,
}

fn parse_color(s: &str) -> IResult<&str, &str> {
    let (s, (c0, c1)) = separated_pair(alpha1, tag(" "), alpha1)(s)?;

    // I'm a bad boy for using unsafe
    let c = unsafe {
        std::str::from_utf8_unchecked(slice::from_raw_parts(c0.as_ptr(), c0.len() + 1 + c1.len()))
    };

    Ok((s, c))
}

fn parse_bag_entry(s: &str) -> IResult<&str, (u32, &str)> {
    let (s, n) = terminated(u32, char(' '))(s)?;
    let (s, c) = parse_color(s)?;
    let (s, _) = alt((tag(" bags"), tag(" bag")))(s)?;

    Ok((s, (n, c)))
}

fn parse_bag_list(s: &str) -> IResult<&str, Vec<(u32, &str)>> {
    separated_list1(tag(", "), parse_bag_entry)(s)
}

fn parse_no_other_bags(s: &str) -> IResult<&str, Vec<(u32, &str)>> {
    tag("no other bags").map(|_| vec![]).parse(s)
}

fn parse_line(line: &str) -> IResult<&str, (&str, Vec<(u32, &str)>)> {
    let (s, c) = parse_color(line)?;
    let (s, _) = tag(" bags contain ")(s)?;

    let (s, contains) = alt((parse_no_other_bags, parse_bag_list))(s)?;

    Ok((s, (c, contains)))
}

fn parse(s: &str) -> Vec<(&str, Vec<(u32, &str)>)> {
    let lines = s.lines().collect::<Vec<_>>();
    lines
        .iter()
        .map(|line| parse_line(line).unwrap().1)
        .collect()
}

fn count_bags(lookup: &HashMap<&str, Vec<(u32, &str)>>, bag: &str) -> u32 {
    let Some(inner) = lookup.get(bag) else { panic!() };
    1 + inner
        .iter()
        .map(|(c, x)| c * count_bags(lookup, x))
        .sum::<u32>()
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Self {
        Instance {
            bags: parse(input).into_iter().collect::<HashMap<_, _>>(),
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut parents = self
            .bags
            .keys()
            .map(|bag| (*bag, Vec::new()))
            .collect::<HashMap<&str, Vec<&str>>>();

        for (parent, children) in self.bags.iter() {
            for (_, child) in children {
                if let Some(ps) = parents.get_mut(child) {
                    ps.push(parent);
                }
            }
        }

        let mut visited = HashSet::<&str>::new();
        let mut queue = vec!["shiny gold"];

        while let Some(n) = queue.pop() {
            if visited.contains(n) {
                continue;
            }

            visited.insert(n);

            queue.extend_from_slice(parents.get(n).unwrap());
        }

        visited.len() - 1
    }

    fn solve_part_two(&self) -> usize {
        count_bags(&self.bags, "shiny gold") as usize - 1
    }
}
