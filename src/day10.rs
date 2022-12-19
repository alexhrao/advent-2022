use regex::Regex;

use crate::utils::get_input;

pub fn task1() {
    let mut cycle: i128 = 1;
    let inp = get_input(10);
    let mut x: i128 = 1;
    let mut v = 0;
    let mut insts = inp.lines();
    let mut is_waiting = false;
    let mut witnesses = vec![];

    let addx_re = Regex::new(r"^addx (-?\d+)$").unwrap();
    loop {
        if (20..=220).step_by(40).find(|&c| c == cycle).is_some() {
            witnesses.push(x * cycle);
        }
        if !is_waiting {
            if let Some(inst) = insts.next() {
                if let Some(caps) = addx_re.captures(inst) {
                    v = caps[1].parse::<i128>().unwrap();
                    is_waiting = true;
                }
            } else {
                break;
            }
        } else {
            x += v;
            is_waiting = false;
        }
        cycle += 1;
    }
    println!("{}", witnesses.iter().sum::<i128>());
}

pub fn task2() {
    let mut cycle: i128 = 1;
    let inp = get_input(10);
    let mut x: i128 = 1;
    let mut v = 0;
    let mut insts = inp.lines();
    let mut is_waiting = false;
    let mut pixels = ['*'; 240];

    let addx_re = Regex::new(r"^addx (-?\d+)$").unwrap();
    loop {
        if cycle > 240 {
            break;
        }
        pixels[(cycle - 1) as usize] = if ((x - 1)..=(x + 1)).contains(&((cycle - 1) % 40)) {
            '#'
        } else {
            '.'
        };
        if !is_waiting {
            if let Some(inst) = insts.next() {
                if let Some(caps) = addx_re.captures(inst) {
                    v = caps[1].parse::<i128>().unwrap();
                    is_waiting = true;
                }
            } else {
                break;
            }
        } else {
            x += v;
            is_waiting = false;
        }
        cycle += 1;
    }

    for r in 0..6 {
        println!(
            "{}",
            pixels[(r * 40)..((r + 1) * 40)].iter().collect::<String>()
        );
    }
}
