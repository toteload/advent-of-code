use crate::Problem;

pub struct Instance {
    bitmap: Vec<bool>,
    width: usize,
    height: usize,
}

impl Instance {
    fn traverse(&self, dx: usize, dy: usize) -> usize {
        let mut count = 0;
        for i in 1..(self.height / dy) {
            let x = (i * dx) % self.width;
            let y = i * dy;

            if self.bitmap[y * self.width + x] {
                count += 1;
            }
        }

        count
    }
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();

        let width = lines[0].len();
        let height = lines.len();

        let bitmap = lines
            .iter()
            .flat_map(|line| line.as_bytes().iter().map(|x| *x == b'#'))
            .collect();

        Instance {
            bitmap,
            width,
            height,
        }
    }

    fn solve_part_one(&self) -> usize {
        self.traverse(3, 1)
    }

    fn solve_part_two(&self) -> usize {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        slopes
            .map(|(dx, dy)| self.traverse(dx, dy))
            .iter()
            .copied()
            .reduce(|acc, x| acc * x)
            .unwrap()
    }
}
