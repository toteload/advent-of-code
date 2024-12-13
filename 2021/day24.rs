enum Operand {
    Reg(usize),
    Const(isize),
}

enum Instruction {
    Inp(usize),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

fn main() {
    let instructions: Vec<Instruction>; 


}
