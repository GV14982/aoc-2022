use std::{collections::HashMap, fs};


fn main() {
    let file = fs::read_to_string("./input.txt").expect("Should read file");
    let sum = part1(&file);
    println!("{}", sum);
    let sum2 = part2(&file);
    println!("{}", sum2);
}

fn build_opponent_sign_lookup_table() -> HashMap<&'static str, &'static str> {
    let mut lookup = HashMap::new();
    lookup.insert("A", "rock");
    lookup.insert("B", "paper");
    lookup.insert("C", "scissors");
    return lookup;
}

fn build_my_sign_lookup_table() -> HashMap<&'static str, &'static str> {
    let mut lookup = HashMap::new();
    lookup.insert("X", "rock");
    lookup.insert("Y", "paper");
    lookup.insert("Z", "scissors");
    return lookup;
}

fn build_win_lookup_table() -> HashMap<&'static str, &'static str> {
    let mut lookup = HashMap::new();
    lookup.insert("rock", "scissors");
    lookup.insert("paper", "rock");
    lookup.insert("scissors", "paper");
    return lookup;
}

fn build_loss_lookup_table() -> HashMap<&'static str, &'static str> {
    let mut lookup = HashMap::new();
    lookup.insert("rock", "paper");
    lookup.insert("paper", "scissors");
    lookup.insert("scissors", "rock");
    return lookup;
}

fn build_point_lookup_table() -> HashMap<&'static str, u32> {
    let mut point_map: HashMap<&str, u32> = HashMap::new();
    point_map.insert("rock", 1);
    point_map.insert("paper", 2);
    point_map.insert("scissors", 3);
    return point_map;
}

fn get_result_points(me: &str, them: &str, win_table: &HashMap<&'static str, &'static str>, point_table: &HashMap<&'static str, u32>) -> u32 {
    let point_val = *point_table.get(me).unwrap();
    // award 3 + sign points for draw
    if me == them {
        return point_val + 3
    }
    // award 6 + sign points for win
    if win_table.get(me).expect("should exist in map") == &them {
        return point_val + 6
    }
    // award sign points for loss
    return point_val;
}

fn part1(arg: &String) -> u32 {
    let my_lookup_table: HashMap<&'static str, &'static str> = build_my_sign_lookup_table();
    let opp_lookup_table: HashMap<&'static str, &'static str> = build_opponent_sign_lookup_table();
    let win_table: HashMap<&'static str, &'static str> = build_win_lookup_table();
    let point_table: HashMap<&'static str, u32> = build_point_lookup_table();
    arg.trim().split("\n")
        .map(|round| {
            let mut split_round = round.trim().split(" ");
            let (them, me) = (opp_lookup_table.get(split_round.next().unwrap()).unwrap(), my_lookup_table.get(split_round.next().unwrap()).unwrap());
            return get_result_points(me, them, &win_table, &point_table)
        })
        .sum::<u32>()
}

fn part2(arg: &String) -> u32 {
    let my_lookup_table: HashMap<&'static str, &'static str> = build_my_sign_lookup_table();
    let opp_lookup_table: HashMap<&'static str, &'static str> = build_opponent_sign_lookup_table();
    let win_table: HashMap<&'static str, &'static str> = build_win_lookup_table();
    let loss_table: HashMap<&'static str, &'static str> = build_loss_lookup_table();
    let point_table: HashMap<&'static str, u32> = build_point_lookup_table();
    arg.trim().split("\n")
        .map(|round| {
            let mut split_round = round.trim().split(" ");
            let (them, outcome) = (opp_lookup_table.get(split_round.next().unwrap()).unwrap(), split_round.next().unwrap());
            let me: &str;
            match outcome {
                "X" => me = win_table.get(them).unwrap(),
                "Y" => me = them,
                "Z" => me = loss_table.get(them).unwrap(),
                _ => panic!("lol whoops")
            }
            println!("{}, {}, {}", them, outcome, me);
            return get_result_points(me, them, &win_table, &point_table)
        })
        .sum::<u32>()
}
