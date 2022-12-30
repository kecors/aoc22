use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let values: Vec<Option<u32>> = input
        .lines()
        .map(|line| match line {
            "" => None,
            _ => Some(line.trim().parse::<u32>().unwrap()),
        })
        .collect();

    let (_total, max_total) =
        values
            .iter()
            .fold((0, 0), |(total, max_total), value_opt| match value_opt {
                None => (0, if total > max_total { total } else { max_total }),
                Some(value) => (total + value, max_total),
            });

    println!("Part 1: that elf is carrying {} calories", max_total);
}
