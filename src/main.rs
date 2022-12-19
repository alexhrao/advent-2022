mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;
fn main() {
    match std::env::args().last().unwrap().parse::<u8>().unwrap() {
        1 => {
            day1::task1();
            day1::task2();
        }
        2 => {
            day2::task1();
            day2::task2();
        }
        3 => {
            day3::task1();
            day3::task2();
        }
        4 => {
            day4::task1();
            day4::task2();
        }
        5 => {
            day5::task1();
            day5::task2();
        }
        6 => {
            day6::task1();
            day6::task2();
        }
        7 => {
            day7::task1();
            day7::task2();
        }
        8 => {
            day8::task1();
            day8::task2();
        }
        9 => {
            day9::task1();
            day9::task2();
        }
        10 => {
            day10::task1();
            day10::task2();
        }
        x => todo!("Day {}", x),
    }
}
