use crate::Problem;
use fixedbitset::FixedBitSet;
use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{line_ending, u32},
    multi::separated_list0,
    sequence::{pair, separated_pair, terminated},
    IResult,
};
use std::ops::RangeInclusive;

struct Field {
    vals: (RangeInclusive<u32>, RangeInclusive<u32>),
}

type Ticket = Vec<u32>;

pub struct Instance {
    fields: Vec<Field>,
    own_ticket: Ticket,
    tickets: Vec<Ticket>,
}

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (s, (start, end)) = separated_pair(u32, tag("-"), u32)(s)?;
    Ok((s, start..=end))
}

fn parse_field(s: &str) -> IResult<&str, Field> {
    let (s, _name) = terminated(take_while(|c| c != ':'), tag(": "))(s)?;
    let (s, vals) = separated_pair(parse_range, tag(" or "), parse_range)(s)?;

    Ok((s, Field { vals }))
}

fn parse_own_ticket(s: &str) -> IResult<&str, Ticket> {
    let (s, _) = terminated(tag("your ticket:"), line_ending)(s)?;
    separated_list0(tag(","), u32)(s)
}

fn parse_tickets(s: &str) -> IResult<&str, Vec<Ticket>> {
    let (s, _) = terminated(tag("nearby tickets:"), line_ending)(s)?;
    separated_list0(line_ending, separated_list0(tag(","), u32))(s)
}

fn parse(s: &str) -> IResult<&str, Instance> {
    let mut empty_line_delimiter = pair(line_ending, line_ending);
    let (s, fields) = terminated(
        separated_list0(line_ending, parse_field),
        &mut empty_line_delimiter,
    )(s)?;

    let (s, own_ticket) = terminated(parse_own_ticket, &mut empty_line_delimiter)(s)?;

    let (s, tickets) = parse_tickets(s)?;

    Ok((
        s,
        Instance {
            fields,
            own_ticket,
            tickets,
        },
    ))
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        parse(input).unwrap().1
    }

    fn solve_part_one(&self) -> usize {
        let mut invalid_values = Vec::<u32>::new();

        for ticket in &self.tickets {
            for v in ticket {
                let is_valid = self
                    .fields
                    .iter()
                    .any(|Field { vals: (a, b) }| a.contains(v) || b.contains(v));

                if !is_valid {
                    invalid_values.push(*v);
                }
            }
        }

        invalid_values.iter().map(|x| *x as usize).sum()
    }

    fn solve_part_two(&self) -> usize {
        // Tickets that have only valid fields.
        let valid_tickets = self
            .tickets
            .iter()
            .filter(|ticket| {
                ticket.iter().all(|v| {
                    self.fields
                        .iter()
                        .any(|Field { vals: (a, b) }| a.contains(v) || b.contains(v))
                })
            })
            .collect::<Vec<_>>();

        // For each of the fields, keep track of the possible mappings to input
        // columns in the tickets.
        let field_count = self.fields.len();
        let mut field_options = std::iter::repeat(FixedBitSet::with_capacity(field_count))
            .map(|mut s| {
                s.set_range(.., true);
                s
            })
            .take(field_count)
            .collect::<Vec<FixedBitSet>>();

        // Eliminate possible mappings based on valid values for the fields.
        for ticket in valid_tickets {
            for (i, v) in ticket.iter().enumerate() {
                for j in 0..field_count {
                    let Field { vals: (a, b), .. } = &self.fields[j];
                    if !(a.contains(v) || b.contains(v)) {
                        field_options[j].set(i, false);
                    }
                }
            }
        }

        // This is not a general solution, but it might be for the Advent of
        // Code input. It is a solution for me at least.
        // At every iteration there will be one field, which will have only
        // one possibility. We can save this field, and then clear this
        // possibility for all the other fields. Iterate until we are done.
        let mut lookup: Vec<Option<usize>> = vec![None; field_count];
        for _ in 0..field_count {
            let idx = field_options
                .iter()
                .position(|x| x.count_ones(..) == 1)
                .unwrap();
            let x = field_options[idx].ones().next();
            lookup[idx] = x;

            for field in field_options.iter_mut() {
                field.set(x.unwrap(), false);
            }
        }

        let lookup = lookup.into_iter().map(|o| o.unwrap()).collect::<Vec<_>>();

        // The first six fields are the "departure" fields.
        lookup[..6]
            .iter()
            .map(|i| self.own_ticket[*i] as usize)
            .product()
    }
}
