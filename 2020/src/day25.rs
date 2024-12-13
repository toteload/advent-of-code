use crate::Problem;

pub struct Instance {
    card_public_key: u32,
    door_public_key: u32,
}

fn transform(subject_number: u32, loop_size: u32) -> u32 {
    let mut acc = 1u64;
    for _ in 0..loop_size {
        acc *= subject_number as u64;
        acc %= 20201227;
    }
    acc as u32
}

impl<'a> Problem<'a> for Instance {
    fn new(input: &'a str) -> Instance {
        Instance {
            card_public_key: 6930903,
            door_public_key: 19716708,
        }
    }

    fn solve_part_one(&self) -> usize {
        let mut acc = 1;
        let mut card_loop_size = None;
        let mut door_loop_size = None;
        for i in 0..u32::MAX {
            if acc == self.card_public_key && card_loop_size.is_none() {
                card_loop_size = Some(i);
            }

            if acc == self.door_public_key && door_loop_size.is_none() {
                door_loop_size = Some(i);
            }

            if card_loop_size.is_some() && door_loop_size.is_some() {
                break;
            }
            acc *= 7;
            acc %= 20201227;
        }

        let card_loop_size = card_loop_size.unwrap();
        let door_loop_size = door_loop_size.unwrap();

        assert_eq!(transform(7, card_loop_size), self.card_public_key);
        assert_eq!(transform(7, door_loop_size), self.door_public_key);

        assert_eq!(
            transform(self.card_public_key, door_loop_size),
            transform(self.door_public_key, card_loop_size)
        );

        println!("card loop size: {}", card_loop_size);
        println!("door loop size: {}", door_loop_size);

        transform(self.card_public_key, door_loop_size) as usize
    }

    fn solve_part_two(&self) -> usize {
        todo!()
    }
}
