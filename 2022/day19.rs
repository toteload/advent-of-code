type Blueprint = [[u16; 4]; 4];

fn recursive_search(blueprint: &Blueprint , max_robots: [u16; 4], ores: [u16; 4], robots: [u16; 4], t: u16, best: &mut u16) {
    let mut searched_deeper = false;

    for i in 0..4 {
        if robots[i] >= max_robots[i] {
            continue;
        }

        let recipe = &blueprint[i];

        let mut wait_time: u16 = 0;
        for j in 0..4 {
            if recipe[j] == 0 {
                continue;
            }

            if recipe[j] <= ores[j] {
                wait_time = wait_time.max(0);
            } else if robots[j] == 0 {
                wait_time = u16::MAX-1;
            } else {
                wait_time = wait_time.max((recipe[j] - ores[j] + robots[j] - 1) / robots[j]);
            }
        }

        if wait_time + 1 >= t {
            continue;
        }

        let mut next_ores = [0; 4];
        for j in 0..4 {
            next_ores[j] = ores[j] + (robots[j] * (wait_time + 1)) - recipe[j];
        }

        let mut next_robots = robots;
        next_robots[i] += 1;

        let rt = t - (wait_time + 1);
        let max_geodes = next_ores[3] + rt * next_robots[3] + ((rt-1) * rt) / 2;
        if max_geodes < *best {
            continue;
        }

        searched_deeper = true;

        recursive_search(blueprint, max_robots, next_ores, next_robots, rt, best);
    }

    if !searched_deeper {
        *best = std::cmp::max(*best, ores[3] + robots[3] * t);
    }
}

fn search(blueprint: &Blueprint, t: u16) -> u16 {
    let mut max_robots = [0,0,0,u16::MAX];
    for i in 0..3 {
        for j in 0..4 {
            max_robots[i] = max_robots[i].max(blueprint[j][i]);
        }
    }

    let mut best: u16 = 0;
    recursive_search(blueprint, max_robots, [0;4], [1,0,0,0], t, &mut best);
    best
}

fn main() {
    let blueprints = [
        [[3,0,0,0], [4,0,0,0], [4,18,0,0], [3,0,8,0]],
        [[2,0,0,0], [4,0,0,0], [4,20,0,0], [3,0,14,0]],
        [[4,0,0,0], [4,0,0,0], [2,11,0,0], [4,0,8,0]],
        [[3,0,0,0], [3,0,0,0], [2,19,0,0], [2,0,20,0]],
        [[4,0,0,0], [3,0,0,0], [4,8,0,0], [3,0,7,0]],
        [[3,0,0,0], [4,0,0,0], [2,11,0,0], [2,0,10,0]],
        [[2,0,0,0], [3,0,0,0], [3,16,0,0], [2,0,11,0]],
        [[4,0,0,0], [3,0,0,0], [4,6,0,0], [3,0,11,0]],
        [[4,0,0,0], [4,0,0,0], [4,8,0,0], [2,0,15,0]],
        [[4,0,0,0], [4,0,0,0], [4,17,0,0], [2,0,13,0]],
        [[2,0,0,0], [4,0,0,0], [4,15,0,0], [2,0,15,0]],
        [[2,0,0,0], [4,0,0,0], [3,20,0,0], [2,0,17,0]],
        [[4,0,0,0], [4,0,0,0], [4,12,0,0], [4,0,19,0]],
        [[2,0,0,0], [3,0,0,0], [3,11,0,0], [2,0,16,0]],
        [[3,0,0,0], [4,0,0,0], [3,16,0,0], [3,0,14,0]],
        [[4,0,0,0], [4,0,0,0], [3,9,0,0], [3,0,7,0]],
        [[2,0,0,0], [3,0,0,0], [2,16,0,0], [2,0,9,0]],
        [[3,0,0,0], [3,0,0,0], [2,14,0,0], [3,0,17,0]],
        [[4,0,0,0], [4,0,0,0], [4,16,0,0], [2,0,15,0]],
        [[2,0,0,0], [4,0,0,0], [4,16,0,0], [4,0,17,0]],
        [[3,0,0,0], [3,0,0,0], [3,16,0,0], [3,0,9,0]],
        [[3,0,0,0], [3,0,0,0], [4,19,0,0], [4,0,7,0]],
        [[3,0,0,0], [3,0,0,0], [3,19,0,0], [3,0,19,0]],
        [[3,0,0,0], [3,0,0,0], [3,8,0,0], [2,0,12,0]],
        [[2,0,0,0], [2,0,0,0], [2,7,0,0], [2,0,14,0]],
        [[4,0,0,0], [3,0,0,0], [2,13,0,0], [2,0,10,0]],
        [[3,0,0,0], [4,0,0,0], [2,20,0,0], [4,0,7,0]],
        [[2,0,0,0], [3,0,0,0], [2,14,0,0], [3,0,8,0]],
        [[4,0,0,0], [4,0,0,0], [3,5,0,0], [3,0,18,0]],
        [[3,0,0,0], [3,0,0,0], [3,16,0,0], [3,0,20,0]],
    ];

    println!("{}", blueprints.iter().enumerate().map(|(i, b)| (i+1) * search(b, 24) as usize).sum::<usize>());
    println!("{}", blueprints.iter().take(3).map(|b| search(b, 32)).reduce(|a,b| a * b).unwrap())
}
