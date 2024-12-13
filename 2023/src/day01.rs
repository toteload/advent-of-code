use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::is_digit,
    combinator::{map, value, verify},
    IResult,
};

pub fn part1(input: &str) -> u32 {
    input.lines().map(|line| {
        let a = line.bytes().find(u8::is_ascii_digit).unwrap() - b'0';
        let b = line.bytes().rfind(u8::is_ascii_digit).unwrap() - b'0';
        (a * 10 + b) as u32
    }).sum()
}

fn written_number(input: &str) -> IResult<&str, u32> {
    alt((
        value(1, tag("one")),
        value(2, tag("two")),
        value(3, tag("three")),
        value(4, tag("four")),
        value(5, tag("five")),
        value(6, tag("six")),
        value(7, tag("seven")),
        value(8, tag("eight")),
        value(9, tag("nine")),
    ))(input)
}

fn a_digit(input: &str) -> IResult<&str, u32> {
    map(
        // There is probably an easier way to read one digit :P
        verify(take(1usize), |s: &str| is_digit(s.as_bytes()[0])),
        |b: &str| (b.as_bytes()[0] - b'0') as u32,
    )(input)
}

pub fn part2(input: &str) -> u32 {
    let mut answer = 0;

    for line in input.lines() {
        let a = 'search: {
            for start in 0..line.len() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    break 'search x;
                }
            }

            unreachable!();
        };

        let b = 'search: {
            for start in (0..line.len()).rev() {
                let res = alt((a_digit, written_number))(&line[start..]);
                if let Ok((_, x)) = res {
                    break 'search x;
                }
            }

            unreachable!();
        };

        answer += a * 10 + b;
    }

    answer
}
