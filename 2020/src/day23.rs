use crate::Problem;

pub struct Instance {
    cups: Vec<u8>,
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        Instance {
            cups: vec![3, 1, 8, 9, 4, 6, 5, 7, 2],
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut acc = [0u8; 9];
        acc.copy_from_slice(&self.cups);

        for _ in 0..100 {
            let mut buf = [0u8; 9];
            let picked_up = &acc[1..4];
            let dst_cup = {
                fn dec(x: u8) -> u8 {
                    if x == 1 {
                        9
                    } else {
                        x - 1
                    }
                }

                let mut dst = dec(acc[0]);
                while picked_up.contains(&dst) {
                    dst = dec(dst);
                }

                dst
            };

            let dst_idx = acc.iter().position(|x| *x == dst_cup).unwrap();
            let front = &acc[4..dst_idx + 1];
            let after_insert = &acc[dst_idx + 1..];

            let offset = front.len();
            buf[..offset].copy_from_slice(front);
            buf[offset..offset + 3].copy_from_slice(picked_up);
            let offset = offset + 3;
            buf[offset..offset + after_insert.len()].copy_from_slice(after_insert);
            buf[8] = acc[0];

            acc = buf;
        }

        let one_idx = acc.iter().position(|x| *x == 1).unwrap();
        acc.rotate_left(one_idx);

        acc.iter()
            .skip(1)
            .rev()
            .enumerate()
            .fold(0usize, |acc, (i, x)| {
                acc + (*x as usize) * 10usize.pow(i as u32)
            })
    }

    fn solve_part_two(&self) -> usize {
        let mut cups = vec![0; 1_000_001];
        for xs in self.cups.windows(2) {
            let [a, b, ..] = xs else { panic!() };
            cups[*a as usize] = *b as usize;
        }

        cups[self.cups[8] as usize] = 10;

        for x in 10..1_000_000 {
            cups[x] = x+1;
        }

        cups[1_000_000] = self.cups[0] as usize;

        let mut current = self.cups[0] as usize;
        for _ in 0..10_000_000 {
            let mut picked_up = [0usize; 3];
            let mut at = current;
            for i in 0..3 {
                let next = cups[at];
                picked_up[i] = next;
                at = next;
            }

            let dst = {
                fn dec(x: usize) -> usize {
                    if x == 1 {
                        1_000_000
                    } else {
                        x - 1
                    }
                }

                let mut dst = dec(current);
                while picked_up.contains(&dst) {
                    dst = dec(dst);
                }

                dst
            };

            let next = cups[at];
            cups[current] = next;
            let after_dst = cups[dst];
            cups[dst] = picked_up[0];
            cups[picked_up[2]] = after_dst;
            current = next;
        }

        let a = cups[1];
        let b = cups[a];

        a * b
    }
}
