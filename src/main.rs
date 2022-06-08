use rand::Rng;
use std::io;

// Return a random number between 1 and the given range
fn dice_roll(num_sides: i32,  modifier: i32) -> (i32, i32, i32) {
    let roll = rand::thread_rng().gen_range(1..num_sides+1);
    let sum = roll+modifier;
    (roll, modifier, sum)
}

// Return a vector of random numbers within the given range and the sum of the vector
fn multi_dice_roll(num_dice: i32, num_sides: i32, modifier: i32) -> (Vec<i32>, i32, i32) {
    let mut results: Vec<i32> = Vec::new();
    
    for _ in 1..num_dice+1 {
        results.push(dice_roll(num_sides, 0).0);
    }

    let sum: i32 = results.iter().sum();

    (results, modifier, sum+modifier)
}

// Print a single roll to the console
fn print_single_roll(roll: (i32, i32, i32)) {
    println!("\nResult: {}", roll.0);
    println!("Modifier: {}", roll.1);
    println!("Sum: {}", roll.2)
}

// Print multiple rolls and their sum to the console
fn print_multi_roll(rolls: (Vec<i32>, i32, i32)) {
    println!("\nResults: {:?}", rolls.0);
    println!("Modifier: {}", rolls.1);
    println!("Sum: {}", rolls.2)
}

// Process the user's input string and return usable elements
fn get_user_input() -> Vec<i32> {
    let mut command = String::new();
    let mut nums: Vec<i32> = Vec::new();

    println!("\nRoll the dice:");

    io::stdin().read_line(&mut command).expect("Failed to read input...");

    let numstrings: Vec<&str> = command.split(&['d', '-', '+'][..]).collect();
    let input: Vec<&str> = command.split_inclusive(&['d', '-', '+'][..]).collect();

    for n in numstrings {
        nums.push(match n.trim().parse() {
            Ok(num) => num,
            Err(_) => quit::with_code(0),
        });
    }

    let i1: Vec<char> = input[1].chars().collect();

    if i1.contains(&'-') {
        nums.splice(2..3, [-nums[2]]);
    }

    if nums.len() < 3 {
        nums.push(0);
    }

    nums
}

#[quit::main]
fn main() {
    loop {
        let nums = get_user_input();

        if nums.len() > 1 {
            if nums[0] == 1 {
                print_single_roll(dice_roll(nums[1], nums[2]));
            } else {
                print_multi_roll(multi_dice_roll(nums[0], nums[1], nums[2]));
            }
        }
    }
}
