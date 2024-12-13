use crate::Problem;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, line_ending, one_of, u64},
    multi::{fold_many0, fold_many1, separated_list0},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub struct Instance<'a> {
    input: &'a str,
}

fn parse_atom(s: &str) -> IResult<&str, u64> {
    let res @ (s, val) = alt((delimited(char('('), parse_expr, char(')')), u64))(s)?;
    Ok(res)
}

fn parse_next(s: &str) -> IResult<&str, (char, u64)> {
    let res @ (s, (op, x)) = pair(delimited(char(' '), one_of("+*"), char(' ')), parse_atom)(s)?;
    Ok(res)
}

fn parse_expr(s: &str) -> IResult<&str, u64> {
    let (s, first) = parse_atom(s)?;
    fold_many1(
        parse_next,
        move || first,
        |acc, (op, x)| match op {
            '+' => acc + x,
            '*' => acc * x,
            _ => unreachable!(),
        },
    )(s)
}

// Part two parsing
// ----------------

fn parse_atom2(s: &str) -> IResult<&str, u64> {
    let res @ (s, val) = alt((delimited(char('('), parse_expr2, char(')')), u64))(s)?;
    Ok(res)
}

fn parse_add_expr(s: &str) -> IResult<&str, u64> {
    let (s, first) = parse_atom2(s)?;
    fold_many0(
        preceded(tag(" + "), parse_atom2),
        move || first,
        |acc, x| acc + x,
    )(s)
}

fn parse_expr2(s: &str) -> IResult<&str, u64> {
    let (s, first) = parse_add_expr(s)?;
    fold_many0(
        preceded(tag(" * "), parse_add_expr),
        move || first,
        |acc, x| acc * x,
    )(s)
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Instance {
        Instance { input }
    }

    fn solve_part_one(&self) -> usize {
        let vals = separated_list0(line_ending, parse_expr)(self.input)
            .unwrap()
            .1;
        vals.iter().sum::<u64>() as usize
    }

    fn solve_part_two(&self) -> usize {
        separated_list0(line_ending, parse_expr2)(self.input)
            .unwrap()
            .1
            .iter()
            .sum::<u64>() as usize
    }
}
