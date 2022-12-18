use crate::utils::get_input;
use regex::Regex;

pub fn task1() {
    let inp = get_input(5);
    let idx = inp.lines().position(|l| l.is_empty()).unwrap();
    let starting: Vec<String> = inp
        .lines()
        .take(idx - 1)
        .map(|s| String::from(s) + " ")
        .collect();
    let insts: Vec<&str> = inp.lines().skip(idx + 1).collect();
    let mut stacks: Vec<String> = vec![String::new(); starting[0].len() / 4];
    for stack_line in starting {
        for (s, b) in stack_line.as_bytes().chunks(4).enumerate() {
            let c = b[1];
            if c != b' ' {
                stacks[s].push(c as char);
            }
        }
    }
    let mut stacks: Vec<String> = stacks
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for inst in insts {
        let caps = re.captures(inst).unwrap();
        let num: usize = caps[1].parse().unwrap();
        let from = &mut stacks[caps[2].parse::<usize>().unwrap() - 1];
        let popped: String = from.drain((from.len() - num)..).rev().collect();
        let to = &mut stacks[caps[3].parse::<usize>().unwrap() - 1];
        to.push_str(&popped);
    }
    for stack in stacks.iter() {
        print!("{}", stack.chars().last().unwrap_or(' '));
    }
    println!("");
}

pub fn task2() {
    let inp = get_input(5);
    let idx = inp.lines().position(|l| l.is_empty()).unwrap();
    let starting: Vec<String> = inp
        .lines()
        .take(idx - 1)
        .map(|s| String::from(s) + " ")
        .collect();
    let insts: Vec<&str> = inp.lines().skip(idx + 1).collect();
    let mut stacks: Vec<String> = vec![String::new(); starting[0].len() / 4];
    for stack_line in starting {
        for (s, b) in stack_line.as_bytes().chunks(4).enumerate() {
            let c = b[1];
            if c != b' ' {
                stacks[s].push(c as char);
            }
        }
    }
    let mut stacks: Vec<String> = stacks
        .into_iter()
        .map(|s| s.chars().rev().collect())
        .collect();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for inst in insts {
        let caps = re.captures(inst).unwrap();
        let num: usize = caps[1].parse().unwrap();
        let from = &mut stacks[caps[2].parse::<usize>().unwrap() - 1];
        let popped: String = from.drain((from.len() - num)..).collect();
        let to = &mut stacks[caps[3].parse::<usize>().unwrap() - 1];
        to.push_str(&popped);
    }
    for stack in stacks.iter() {
        print!("{}", stack.chars().last().unwrap_or(' '));
    }
    println!("");
}
