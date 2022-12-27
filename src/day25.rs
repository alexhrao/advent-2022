use radix_fmt::radix_5;

use crate::utils::get_input;

fn snafu_to_dec(snafu: &str) -> isize {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(place, c)| {
            5isize.pow(place as u32)
                * match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!(),
                }
        })
        .sum()
}

fn dec_to_snafu(dec: isize) -> String {
    let mut carry: u8 = 0;
    let mut out = String::new();

    for c in radix_5(dec).to_string().bytes().rev() {
        let c = c + carry;
        if c == '0' as u8 || c == '1' as u8 || c == '2' as u8 {
            out.push(c as char);
            carry = 0;
        } else if c == '3' as u8 {
            out.push('=');
            carry = 1;
        } else if c == '4' as u8 {
            out.push('-');
            carry = 1;
        } else {
            out.push('0');
            carry = 1;
        }
    }
    if carry != 0 {
        out.push('1');
    }

    out.chars().rev().collect()
}

pub fn task1() {
    let a = dec_to_snafu(get_input(25).lines().map(|l| snafu_to_dec(l)).sum());
    println!("{}", a);
    todo!();
}

pub fn task2() {
    todo!();
}
