const text = Deno.readTextFileSync('day02.txt');

const initMemory = text.split(',').map(x => parseInt(x, 10));

function run(mem: number[], a: number, b: number): number {
    mem[1] = a;
    mem[2] = b;

    let pc = 0;
    while (true) {
        const op = mem[pc];

        switch (op) {
        case 1: {
            const a = mem[mem[pc+1]];
            const b = mem[mem[pc+2]];
            const dst = mem[pc+3];
            mem[dst] = a + b;
            pc += 4;
        } break;
        case 2: {
            const a = mem[mem[pc+1]];
            const b = mem[mem[pc+2]];
            const dst = mem[pc+3];
            mem[dst] = a * b;
            pc += 4;
        } break;
        case 99: {
            return mem[0];
        }
        }
    }
}


function findNeedle(needle: number): number {
    for (let i = 0; i < 100; i++) {
        for (let j = 0; j < 100; j++) {
            if (run([...initMemory], i, j) === needle) {
                return (100 * i + j);
            }
        }
    }
    return 0;
}

const needle = 19690720;
console.log(findNeedle(needle))
