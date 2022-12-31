use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut values: Vec<Option<u32>> = input
        .lines()
        .map(|line| match line {
            "" => None,
            _ => Some(line.trim().parse::<u32>().unwrap()),
        })
        .collect();
    // Mark end of last group
    values.push(None);

    // Calculate totals for each group
    let mut totals = Vec::new();
    values.iter().fold(0, |total, value_opt| match value_opt {
        None => {
            totals.push(total);
            0
        }
        Some(value) => total + value,
    });
    totals.sort();

    // Part 1
    let max_total: &u32 = totals.iter().last().unwrap();
    println!("Part 1: that elf is carrying {} calories", max_total);

    // Part 2
    let sum: u32 = totals.iter().rev().take(3).sum();
    println!("Part 2: those elves are carrying {} calories", sum);
}
