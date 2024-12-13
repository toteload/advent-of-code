use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

struct Instance {
    enhancement_string: Vec<bool>,
    input: Image,
}

impl Instance {
    fn from_text(s: &str) -> Instance {
        let mut lines = s.lines();
        let enhancement_string = lines
            .next()
            .unwrap()
            .trim_end()
            .bytes()
            .map(|i| i == b'#')
            .collect();

        // Skip the empty line
        lines.next();

        let image_lines = lines
            .map(|s| s.trim_end().as_bytes())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let width = image_lines[0].len() as i32;
        let height = image_lines.len() as i32;

        let mut pixels = HashSet::new();

        for y in 0..height {
            for x in 0..width {
                if image_lines[y as usize][x as usize] == b'#' {
                    pixels.insert((x, y));
                }
            }
        }

        Instance {
            enhancement_string,
            input: Image {
                pixels,
                dim: RangeBox::new(&(0..width), &(0..height)),
                is_border_lit: false,
            },
        }
    }
}

#[derive(Clone)]
struct RangeBox {
    range: [Range<i32>; 2],
}

impl RangeBox {
    fn new(width: &Range<i32>, height: &Range<i32>) -> RangeBox {
        RangeBox {
            range: [width.clone(), height.clone()],
        }
    }

    fn shrink(&self) -> RangeBox {
        RangeBox {
            range: self.range.clone().map(|r| (r.start + 1)..(r.end - 1)),
        }
    }

    fn grow(&self) -> RangeBox {
        RangeBox {
            range: self.range.clone().map(|r| (r.start - 1)..(r.end + 1)),
        }
    }

    fn width(&self) -> Range<i32> {
        self.range[0].clone()
    }

    fn height(&self) -> Range<i32> {
        self.range[1].clone()
    }

    fn contains(&self, p: &(i32, i32)) -> bool {
        self.range[0].contains(&p.0) && self.range[1].contains(&p.1)
    }
}

#[derive(Clone)]
struct Image {
    pixels: HashSet<(i32, i32)>,
    dim: RangeBox,
    is_border_lit: bool,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.dim.height() {
            for x in self.dim.width() {
                if self.pixels.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn sample_points(x: i32, y: i32) -> [(i32, i32); 9] {
    [
        (x + 1, y + 1),
        (x, y + 1),
        (x - 1, y + 1),
        (x + 1, y),
        (x, y),
        (x - 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
    ]
}

impl Image {
    fn enhance(&self, enhancement_string: &[bool]) -> Image {
        let mut pixels = HashSet::new();

        let inner_bounds = self.dim.shrink();

        for x in inner_bounds.width() {
            for y in inner_bounds.height() {
                let mut idx = 0usize;
                for (i, p) in sample_points(x, y).iter().enumerate() {
                    if self.pixels.contains(p) {
                        idx |= 1 << i;
                    }
                }

                if enhancement_string[idx] {
                    pixels.insert((x, y));
                }
            }
        }

        let outer_bounds = self.dim.grow();

        for y in outer_bounds.height() {
            for x in outer_bounds.width() {
                if inner_bounds.contains(&(x, y)) {
                    continue;
                }

                let mut idx = 0usize;
                for (i, p) in sample_points(x, y).iter().enumerate() {
                    let is_in_image = self.dim.contains(p);
                    let is_lit = if is_in_image {
                        self.pixels.contains(p)
                    } else {
                        self.is_border_lit
                    };
                    if is_lit {
                        idx |= 1 << i;
                    }
                }

                if enhancement_string[idx] {
                    pixels.insert((x, y));
                }
            }
        }

        Image {
            pixels,
            dim: outer_bounds,
            is_border_lit: enhancement_string[if self.is_border_lit { 511 } else { 0 }],
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let instance = Instance::from_text(input);

    let mut image = instance.input.clone();
    for _ in 0..50 {
        image = image.enhance(&instance.enhancement_string);
    }

    println!("{}", image.pixels.len());
}
