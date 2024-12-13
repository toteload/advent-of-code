use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, line_ending, u32},
    multi::{count, separated_list1},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use crate::Problem;

pub struct Instance<'a> {
    rules: Vec<Rule>,
    messages: Vec<&'a str>,
}

#[derive(Clone)]
enum Rule {
    Seq(Vec<usize>),
    Or(Vec<Vec<usize>>),
    Lit(char),
}

fn parse_rule(s: &str) -> IResult<&str, Rule> {
    let lit = delimited(char('"'), anychar, char('"')).map(|x| Rule::Lit(x));
    let seq = separated_list1(char(' '), u32.map(|x| x as usize)).map(|x| Rule::Seq(x));
    let or = separated_list1(tag(" | "), separated_list1(char(' '), u32.map(|x| x as usize))).map(|x| Rule::Or(x));
    alt((or, seq, lit))(s)
}

fn parse_rule_line(line: &str) -> IResult<&str, (usize, Rule)> {
    separated_pair(u32.map(|x| x as usize), tag(": "), parse_rule)(line)
}

fn parse_rules(s: &str) -> IResult<&str, Vec<(usize, Rule)>> {
    separated_list1(line_ending, parse_rule_line)(s)
}

fn parse_messages(s: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(line_ending, alpha1)(s)
}

fn parse(s: &str) -> (Vec<Rule>, Vec<&str>) {
    let (idx_rules, messages) =
        separated_pair(parse_rules, count(line_ending, 2), parse_messages)(s)
            .unwrap()
            .1;

    let mut rules = vec![Rule::Lit(' '); idx_rules.len()];
    for (idx, rule) in idx_rules.iter() {
        rules[*idx] = rule.clone();
    }

    (rules, messages)
}

fn does_match<'a>(rules: &[Rule], rule: &Rule, msg: &'a str) -> Option<&'a str> {
    use Rule::*;
    match rule {
        Lit(c) => {
            let bs = msg.as_bytes();
            if bs[0] == *c as u8 {
                Some(std::str::from_utf8(&bs[1..]).unwrap())
            } else {
                None
            }
        }
        Seq(xs) => {
            let mut acc = msg;
            for x in xs {
                if let Some(rest) = does_match(rules, &rules[*x], acc) {
                    acc = rest
                } else {
                    return None;
                }
            }

            Some(acc)
        }
        Or(xs) => {
            for branch in xs {
                let rs = Rule::Seq(branch.clone());
                if let res @ Some(_) = does_match(rules, &rs, msg) {
                    return res;
                }
            }

            None
        }
    }
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Instance<'a> {
        let (rules, messages) = parse(input);
        Instance { rules, messages }
    }

    fn solve_part_one(&self) -> usize {
        self.messages
            .iter()
            .filter(|msg| {
                if let Some(rest) = does_match(&self.rules, &self.rules[0], msg) {
                    rest.is_empty()
                } else {
                    false
                }
            })
            .count()
    }

    fn solve_part_two(&self) -> usize {
        let mut rules = self.rules.clone();
        rules[8] = Rule::Or(vec![vec![42], vec![42, 8]]);
        rules[11] = Rule::Or(vec![vec![42, 31], vec![42, 11, 31]]);
        todo!()
    }
}
