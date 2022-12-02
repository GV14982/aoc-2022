use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").expect("Should read file");
    let max = part1(&file);
    let top3 = part2(&file);
    println!("{}", max);
    println!("{}", top3);
}

fn part1(arg: &String) -> u32 {
    return arg
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split("\n")
                .map(|cals| cals.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .max()
        .unwrap_or(0);
}

fn part2(arg: &String) -> u32 {
    return arg
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split("\n")
                .map(|cals| cals.parse::<u32>().unwrap_or(0))
                .sum::<u32>()
        })
        .fold(vec![0, 0, 0], |top3, curr| {
            let mut result = top3.clone();
            if curr > result[0] {
                result[0] = curr;
            }
            result.sort_unstable();
            return result;
        })
        .iter()
        .sum::<u32>();
}
