use regex::Regex;

use crate::utils::get_input;

struct Monkey {
    pub id: usize,
    pub items: Vec<u128>,
    pub op: Box<dyn Fn(u128) -> u128>,
    pub test: u128,
    pub throw: (usize, usize),
}

pub fn task1() {
    let id_re = Regex::new(r"^Monkey (\d+):$").unwrap();
    let items_re = Regex::new(r"Starting items: ([\d, ]+)").unwrap();
    let op_re = Regex::new(r"Operation: new = (\d+|old) ([*+\-]) (\d+|old)$").unwrap();
    let test_re = Regex::new(r"divisible by (\d+)$").unwrap();
    let throw_re = Regex::new(r"If (?:true|false): throw to monkey (\d+)$").unwrap();

    let mut monkeys: Vec<Monkey> = get_input(11)
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|m| {
            let id = id_re.captures(m[0]).unwrap()[1].parse().unwrap();
            let starting_items = &items_re.captures(m[1]).unwrap()[1];
            let items = starting_items
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();
            let op_caps = op_re.captures(m[2]).unwrap();
            let op_l = op_caps[1].parse().ok();
            let op_r = op_caps[3].parse().ok();
            //println!("{:?}", op_r);
            let op: Box<dyn Fn(u128) -> u128> = match &op_caps[2] {
                "*" => Box::new(move |x: u128| op_l.unwrap_or(x) * op_r.unwrap_or(x)),
                "+" => Box::new(move |x: u128| op_l.unwrap_or(x) + op_r.unwrap_or(x)),
                _ => panic!(),
            };
            let test = test_re.captures(m[3]).unwrap()[1].parse().unwrap();
            let throw_true = throw_re.captures(m[4]).unwrap()[1].parse().unwrap();
            let throw_false = throw_re.captures(m[5]).unwrap()[1].parse().unwrap();
            Monkey {
                id,
                items,
                op,
                test,
                throw: (throw_true, throw_false),
            }
        })
        .collect();
    let mut stats = vec![0; monkeys.len()];
    let mut monkey_queues = vec![vec![]; monkeys.len()];
    for _ in 0..20 {
        for monkey in &mut monkeys {
            monkey.items.extend(monkey_queues[monkey.id].drain(..));
            stats[monkey.id] += monkey.items.len();
            for item in monkey.items.drain(..) {
                let item = (monkey.op)(item) / 3;
                let idx = if (item % monkey.test) == 0 {
                    monkey.throw.0
                } else {
                    monkey.throw.1
                };
                monkey_queues[idx].push(item);
            }
        }
    }

    stats.sort_unstable();
    stats.reverse();
    println!("{}", stats.iter().take(2).product::<usize>());
}

pub fn task2() {
    let id_re = Regex::new(r"^Monkey (\d+):$").unwrap();
    let items_re = Regex::new(r"Starting items: ([\d, ]+)").unwrap();
    let op_re = Regex::new(r"Operation: new = (\d+|old) ([*+\-]) (\d+|old)$").unwrap();
    let test_re = Regex::new(r"divisible by (\d+)$").unwrap();
    let throw_re = Regex::new(r"If (?:true|false): throw to monkey (\d+)$").unwrap();

    let mut monkeys: Vec<Monkey> = get_input(11)
        .lines()
        .collect::<Vec<&str>>()
        .chunks(7)
        .map(|m| {
            let id = id_re.captures(m[0]).unwrap()[1].parse().unwrap();
            let starting_items = &items_re.captures(m[1]).unwrap()[1];
            let items = starting_items
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();
            let op_caps = op_re.captures(m[2]).unwrap();
            let op_l = op_caps[1].parse().ok();
            let op_r = op_caps[3].parse().ok();
            //println!("{:?}", op_r);
            let op: Box<dyn Fn(u128) -> u128> = match &op_caps[2] {
                "*" => Box::new(move |x: u128| op_l.unwrap_or(x) * op_r.unwrap_or(x)),
                "+" => Box::new(move |x: u128| op_l.unwrap_or(x) + op_r.unwrap_or(x)),
                _ => panic!(),
            };
            let test = test_re.captures(m[3]).unwrap()[1].parse().unwrap();
            let throw_true = throw_re.captures(m[4]).unwrap()[1].parse().unwrap();
            let throw_false = throw_re.captures(m[5]).unwrap()[1].parse().unwrap();
            Monkey {
                id,
                items,
                op,
                test,
                throw: (throw_true, throw_false),
            }
        })
        .collect();
    let gcd: u128 = monkeys.iter().map(|m| m.test).product();
    let mut stats = vec![0; monkeys.len()];
    let mut monkey_queues = vec![vec![]; monkeys.len()];
    for _ in 0..10000 {
        for monkey in &mut monkeys {
            monkey.items.extend(monkey_queues[monkey.id].drain(..));
            stats[monkey.id] += monkey.items.len();
            for item in monkey.items.drain(..) {
                let item = (monkey.op)(item) % gcd;
                let idx = if (item % monkey.test) == 0 {
                    monkey.throw.0
                } else {
                    monkey.throw.1
                };
                monkey_queues[idx].push(item);
            }
        }
    }
    stats.sort_unstable();
    stats.reverse();
    println!("{}", stats.iter().take(2).product::<usize>());
}
