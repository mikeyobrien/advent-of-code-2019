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
// Used for day 1
fn process_opcodes(opcodes: &mut Vec<i32>){
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
}

fn day_2() {
    let contents: String = fs::read_to_string("inputs/day2.txt").expect("Unable to open file");
    let base_opcodes: Vec<i32> = contents.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..100 {
        for j in 0..100 {
            let mut opcodes = base_opcodes.clone();
            opcodes[1] = i;
            opcodes[2] = j;
            process_opcodes(&mut opcodes);
            if opcodes[0] == 19690720{
                println!("day 2: {:?}", 100 * opcodes[1] + opcodes[2]);
            }
        }
    }
}

/******************** Day 3 ********************/
fn day_3() {
    println!("day 3: in progress");
}


fn main() {
    day_1();
    day_2();
    day_3();
}
