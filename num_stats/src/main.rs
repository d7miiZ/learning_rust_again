use std::io;
use std::collections::HashMap;

fn main() {
    println!("Enter a list of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut numbers: Vec<i32> = vec![];
    let mut frequency: HashMap<i32, i32> = HashMap::new();
    let mut mode: Vec<i32> = vec![];
    let mut mode_count = 0;

    for number in input.split_whitespace() {
        let number: i32 = number.parse().expect("Please type a number!");
        numbers.push(number);
        let count = frequency.entry(number).or_insert(0);
        *count += 1;
        if *count > mode_count {
            mode_count = *count;
        }
    }

    for (number, count) in &frequency {
        if *count == mode_count {
            mode.push(*number);
        }
    }

    numbers.sort();
    let mid_index = numbers.len() / 2;
    let median = numbers[mid_index];
    println!("The median {median} and the mode {mode:?}");
}
