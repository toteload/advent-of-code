use std::collections::{BTreeSet, HashMap, HashSet};
use std::io::{self, BufRead};

fn part_one(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|line| line.split('|').nth(1).unwrap().trim())
        .map(|output| {
            output
                .split_whitespace()
                .filter(|&x| match x.len() {
                    2 | 3 | 4 | 7 => true,
                    _ => false,
                })
                .count()
        })
        .sum()
}

fn make_set(elems: &[char]) -> HashSet<char> {
    let mut set = HashSet::new();

    for e in elems {
        set.insert(e.clone());
    }

    set
}

fn decode_output(line: &str) -> usize {
    // Dear God... This solution is ugly :P
    // It pretty much worked first try, but there definitely exists a solution
    // that is less work/more elegant/more general/less hardcoded.
    //
    //
    //  aaaa
    // b    c
    // b    c
    //  dddd
    // e    f
    // e    f
    //  gggg
    //
    //
    // This approach figures out which segment in the input maps to the segment
    // layout shown above using some hardcoded logic. Then a map is created
    // where each digit is uniquely identified by a set of segments. For each
    // digit in the output we use this map to look up its value.

    let mut parts = line.split('|');
    let numbers_part = parts.next().unwrap();
    let output_part = parts.next().unwrap();

    let numbers = numbers_part
        .split_whitespace()
        .map(|s| {
            let mut set = HashSet::new();
            for c in s.chars() {
                set.insert(c);
            }
            set
        })
        .collect::<Vec<HashSet<char>>>();

    let one = numbers.iter().find(|&x| x.len() == 2).unwrap();
    let four = numbers.iter().find(|&x| x.len() == 4).unwrap();
    let seven = numbers.iter().find(|&x| x.len() == 3).unwrap();
    let eight = numbers.iter().find(|&x| x.len() == 7).unwrap();

    // The a segment is the one that is in 7 but not present in 1.
    let a = *(seven - &one).iter().next().unwrap();

    // 0, 6 and 9 are the only numbers using 6 segments.
    let zero_six_nine: Vec<HashSet<char>> =
        numbers.iter().filter(|&x| x.len() == 6).cloned().collect();

    // The set containing all segments of 4 and segment a
    let four_and_a = {
        let mut s = four.clone();
        s.insert(a);
        s
    };

    // Between 0, 6 and 9, the 9 is the only one with a difference of size 1.
    // This difference is the g segment.
    let pos_nine = zero_six_nine
        .iter()
        .position(|x| (x - &four_and_a).len() == 1)
        .unwrap();

    let nine = &zero_six_nine[pos_nine];

    let g = *(nine - &four_and_a).iter().next().unwrap();

    // The segment difference between 8 and 9 is the e segment.
    let e = *(eight - nine).iter().next().unwrap();

    // Between 0, 6 and 9, the 6 is the only one with a non-empty
    // overlap with 1.
    let pos_six = zero_six_nine
        .iter()
        .position(|x| !(one - x).is_empty())
        .unwrap();

    let six = &zero_six_nine[pos_six];

    let pos_zero = match pos_six + pos_nine {
        1 => 2,
        2 => 1,
        3 => 0,
        _ => unreachable!(),
    };

    // From the set of 0, 6 and 9, we know which ones are 6 and 9 leaving us
    // with only 0.
    let zero = &zero_six_nine[pos_zero];

    let d = *(eight - zero).iter().next().unwrap();

    let c = *(one - six).iter().next().unwrap();

    let b = *(eight - &(&make_set(&[a, c, d, e, g]) | &one))
        .iter()
        .next()
        .unwrap();

    let f = *(eight - &(&make_set(&[a, b, c, d, e, g])))
        .iter()
        .next()
        .unwrap();

    let decoder = HashMap::from([
        ('a', a),
        ('b', b),
        ('c', c),
        ('d', d),
        ('e', e),
        ('f', f),
        ('g', g),
    ]);

    let lookup: HashMap<BTreeSet<char>, usize> = HashMap::from([
        (
            BTreeSet::from(['a', 'b', 'c', 'e', 'f', 'g'].map(|x| decoder[&x])),
            0,
        ),
        (BTreeSet::from(['c', 'f'].map(|x| decoder[&x])), 1),
        (
            BTreeSet::from(['a', 'c', 'd', 'e', 'g'].map(|x| decoder[&x])),
            2,
        ),
        (
            BTreeSet::from(['a', 'c', 'd', 'f', 'g'].map(|x| decoder[&x])),
            3,
        ),
        (BTreeSet::from(['b', 'c', 'd', 'f'].map(|x| decoder[&x])), 4),
        (
            BTreeSet::from(['a', 'b', 'd', 'f', 'g'].map(|x| decoder[&x])),
            5,
        ),
        (
            BTreeSet::from(['a', 'b', 'd', 'e', 'f', 'g'].map(|x| decoder[&x])),
            6,
        ),
        (BTreeSet::from(['a', 'c', 'f'].map(|x| decoder[&x])), 7),
        (
            BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g'].map(|x| decoder[&x])),
            8,
        ),
        (
            BTreeSet::from(['a', 'b', 'c', 'd', 'f', 'g'].map(|x| decoder[&x])),
            9,
        ),
    ]);

    let outs = output_part
        .split_whitespace()
        .map(|s| {
            let mut set = BTreeSet::new();
            for c in s.chars() {
                set.insert(c);
            }
            set
        })
        .collect::<Vec<BTreeSet<char>>>();

    let result = outs
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| lookup[x] * 10usize.pow(i as u32))
        .sum();

    result
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}",
        lines.iter().map(|line| decode_output(line)).sum::<usize>()
    );
}
