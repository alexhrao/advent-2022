mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
//mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod utils;
fn main() {
    match std::env::args().last().unwrap().parse::<u8>().unwrap() {
        1 => {
            day01::task1();
            day01::task2();
        }
        2 => {
            day02::task1();
            day02::task2();
        }
        3 => {
            day03::task1();
            day03::task2();
        }
        4 => {
            day04::task1();
            day04::task2();
        }
        5 => {
            day05::task1();
            day05::task2();
        }
        6 => {
            day06::task1();
            day06::task2();
        }
        7 => {
            day07::task1();
            day07::task2();
        }
        8 => {
            day08::task1();
            day08::task2();
        }
        9 => {
            day09::task1();
            day09::task2();
        }
        10 => {
            day10::task1();
            day10::task2();
        }
        11 => {
            day11::task1();
            day11::task2();
        }
        12 => {
            day12::task1();
            day12::task2();
        }
        13 => {
            day13::task1();
            day13::task2();
        }
        14 => {
            day14::task1();
            day14::task2();
        }
        15 => {
            day15::task1();
            day15::task2();
        }
        17 => {
            day17::task1();
            day17::task2();
        }
        18 => {
            day18::task1();
            day18::task2();
        }
        19 => {
            day19::task1();
        }
        20 => {
            day20::task1();
            day20::task2();
        }
        21 => {
            day21::task1();
            day21::task2();
        }
        22 => {
            day22::task1();
            day22::task2();
        }
        23 => {
            day23::task1();
            day23::task2();
        }
        24 => {
            day24::task1();
            day24::task2();
        }
        25 => {
            day25::task1();
            day25::task2();
        }
        x => todo!("Day {}", x),
    }
}
