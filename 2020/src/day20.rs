use crate::{Bitmap, Problem};
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u32},
    multi::separated_list0,
    sequence::{pair, preceded},
    IResult,
};

struct Tile {
    id: u32,
    val: Bitmap<bool>,
}

pub struct Instance {
    tiles: Vec<Tile>,
}

fn parse_tile(s: &str) -> (u32, Bitmap<bool>) {
    let mut lines = s.lines();
    let (s, id) =
        preceded(tag("Tile "), u32::<_, nom::error::Error<_>>)(lines.next().unwrap()).unwrap();
    let bitmap_lines = lines.collect::<Vec<_>>();
    let width = bitmap_lines[0].len();
    let height = bitmap_lines.len();

    let mut bitmap = Bitmap::new(false, width, height);

    for (y, row) in bitmap_lines.iter().enumerate() {
        for (x, c) in row.as_bytes().iter().enumerate() {
            if *c == b'#' {
                bitmap[(x as isize, y as isize)] = true;
            }
        }
    }

    (id, bitmap)
}

fn parse(s: &str) -> Vec<(u32, Bitmap<bool>)> {
    //let mut empty_line_delimiter = pair(line_ending, line_ending);
    //separated_list0(&mut empty_line_delimiter, parse_tile)(s).unwrap();
    todo!()
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        todo!()
    }

    fn solve_part_one(&self) -> usize {
        todo!()
    }

    fn solve_part_two(&self) -> usize {
        todo!()
    }
}
