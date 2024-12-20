struct Machine {
    reg: [u64; 3],
    pc: usize,
}

impl Machine {
    fn combo(&self, operand: u8) -> u64 {
        if operand <= 3 {
            return operand as u64;
        }

        match operand {
            4 => self.reg[0],
            5 => self.reg[1],
            6 => self.reg[2],
            _ => unreachable!(),
        }
    }

    fn run(&mut self, prog: &[u8], output: &mut Vec<u8>) {
        while self.pc < prog.len() {
            let opcode = prog[self.pc];
            let operand = prog[self.pc + 1];

            self.pc += 2;

            match opcode {
                0 => self.reg[0] /= 1 << self.combo(operand),
                1 => self.reg[1] ^= operand as u64,
                2 => self.reg[1] = self.combo(operand) & 0b111,
                3 => {
                    if self.reg[0] != 0 {
                        self.pc = operand as usize;
                        continue;
                    }
                }
                4 => self.reg[1] ^= self.reg[2],
                5 => output.push((self.combo(operand) & 0b111) as u8),
                6 => self.reg[1] = self.reg[0] / (1 << self.combo(operand)),
                7 => self.reg[2] = self.reg[0] / (1 << self.combo(operand)),
                _ => unreachable!(),
            }
        }
    }
}

fn print_output(output: &[u8]) {
    for x in &output[..output.len() - 1] {
        print!("{},", char::from_digit(*x as u32, 10).unwrap());
    }

    println!(
        "{}",
        char::from_digit(*output.last().unwrap() as u32, 10).unwrap()
    );
}

fn main() {
    // You have to manually add your input here.
    let a = 0;
    let prog = [];

    {
        // Part 1

        let mut machine = Machine {
            reg: [a, 0, 0],
            pc: 0,
        };

        let mut output = Vec::new();

        machine.run(&prog, &mut output);

        print_output(&output);
    }

    {
        // Part 2

        let mut output = Vec::new();
        let mut seed = 1 << (3 * (prog.len() - 1));

        loop {
            output.clear();

            let mut machine = Machine {
                reg: [seed, 0, 0],
                pc: 0,
            };

            machine.run(&prog, &mut output);

            if &output == &prog {
                break;
            }

            let mut i = None;
            for (j, (x, y)) in prog.iter().zip(output.iter()).enumerate().rev() {
                if x != y {
                    i = Some(j);
                    break;
                }
            }

            let i = i.unwrap();

            seed += 1 << (3 * i);
        }

        println!("{seed}");
    }
}
