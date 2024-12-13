import os.path

TEMPLATE = \
"""use crate::Problem;

pub struct Instance {

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
"""

def create_files():
    for i in range(1,26):
        name = f'src/day{i:02}.rs'

        # Skip if the file already exists
        if os.path.isfile(name):
            continue

        with open(name, 'x') as f:
            f.write(TEMPLATE)

for i in range(11,26):
    print(f'mod day{i:02};')