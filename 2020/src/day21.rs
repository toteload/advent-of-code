use crate::Problem;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::{alpha1, char, line_ending},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};
use std::collections::{HashMap, HashSet};

pub struct Instance<'a> {
    products: Vec<(Vec<&'a str>, Vec<&'a str>)>,
}

fn parse_allergens(s: &str) -> IResult<&str, Vec<&str>> {
    let (s, _) = tag("(contains ")(s)?;
    terminated(separated_list1(tag(", "), alpha1), tag(")"))(s)
}

fn parse_product(s: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (s, ingredient_str) = take_till(|c| c == '(')(s)?;
    let (_, ingredients) = separated_list1(char(' '), alpha1)(ingredient_str)?;
    let (s, allergens) = parse_allergens(s)?;
    Ok((s, (ingredients, allergens)))
}

type Product<'a> = (Vec<&'a str>, Vec<&'a str>);

fn parse(s: &str) -> IResult<&str, Vec<Product>> {
    separated_list1(line_ending, parse_product)(s)
}

impl<'a> Problem<'a> for Instance<'a> {
    fn new(input: &'a str) -> Instance<'a> {
        Instance {
            products: parse(input).unwrap().1,
        }
    }

    fn solve_part_one(&self) -> usize {
        let all_allergens = self
            .products
            .iter()
            .flat_map(|(_, allergens)| allergens.clone())
            .collect::<HashSet<_>>();
        let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();

        for (ingredients, allergens) in self.products.iter() {
            for a in allergens {
                let ingredient_set = ingredients.iter().copied().collect::<HashSet<_>>();
                let possible_ingredients = m.entry(*a).or_insert_with(|| ingredient_set.clone());
                *possible_ingredients = &*possible_ingredients & &ingredient_set;
            }
        }

        dbg!(&m);

        let all_ingredients = self
            .products
            .iter()
            .flat_map(|(ingredients, _)| ingredients.clone())
            .collect::<HashSet<_>>();
        let bad_ingredients = m.into_values().fold(HashSet::new(), |acc, s| &acc | &s);
        let safe_ingredients = &all_ingredients - &bad_ingredients;

        let mut count = 0usize;
        for (ingredients, _) in &self.products {
            for i in ingredients {
                if safe_ingredients.contains(i) {
                    count += 1;
                }
            }
        }

        count
    }

    fn solve_part_two(&self) -> usize {
        let all_allergens = self
            .products
            .iter()
            .flat_map(|(_, allergens)| allergens.clone())
            .collect::<HashSet<_>>();
        let mut m: HashMap<&str, HashSet<&str>> = HashMap::new();

        for (ingredients, allergens) in self.products.iter() {
            for a in allergens {
                let ingredient_set = ingredients.iter().copied().collect::<HashSet<_>>();
                let possible_ingredients = m.entry(*a).or_insert_with(|| ingredient_set.clone());
                *possible_ingredients = &*possible_ingredients & &ingredient_set;
            }
        }

        let mut l = Vec::new();

        while !m.is_empty() {
            let (a, i) = m
                .iter()
                .find(|&(_, ingredients)| ingredients.len() == 1)
                .map(|(&allergen, ingredients)| (allergen, *ingredients.iter().next().unwrap()))
                .unwrap();

            l.push((a, i));
            m.remove(a);

            for (_, ingredients) in m.iter_mut() {
                ingredients.remove(i);
            }
        }

        l.sort_by_key(|(a, _)| *a);

        let mut answer = String::new();
        for (_, i) in l {
            answer.push_str(i);
            answer.push(',');
        }

        println!("{}", answer);

        0
    }
}
