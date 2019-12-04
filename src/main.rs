use std::fs;

/******************** Day 1 ********************/
fn calculate_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel < 1 {
        return 0;
    }
    return fuel + calculate_fuel(fuel);
}

fn day_1() {
    let contents = fs::read_to_string("inputs/day1.txt").expect("Unable to open file");
    let sum: i32 = contents
        .lines()
        .map(|x| calculate_fuel(x.parse::<i32>().unwrap()))
        .sum();
    println!("day 1: {}", sum)
}

/******************** Day 2 ********************/
fn day_2() {
    let contents: String = fs::read_to_string("inputs/day2.txt").expect("Unable to open file");
    let mut opcodes: Vec<i32> = contents.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    for i in (0..opcodes.len()).step_by(4) {
        match opcodes[i] {
            1 => {
                let update_index = opcodes[i+3];
                opcodes[update_index as usize] = opcodes[opcodes[i+1] as usize] + opcodes[opcodes[i+2] as usize]
            },
            2 => {
                let update_index = opcodes[i+3];
                opcodes[update_index as usize] = opcodes[opcodes[i+1] as usize] * opcodes[opcodes[i+2] as usize]
            },

            99 => break,
            _ => panic!()
        }
    }
    println!("day 2: {:?}", opcodes[0]);
}


fn main() {
    day_1();
    day_2();
}
