use crate::utils::get_input;

pub fn task1() {
    let mut num_pairs: u128 = 0;
    for pair in get_input(4).lines() {
        let (r1, r2) = pair.split_once(",").unwrap();
        let (n11, n12) = r1.split_once("-").unwrap();
        let (n21, n22) = r2.split_once("-").unwrap();
        let r1 = n11.parse::<u128>().unwrap()..=n12.parse::<u128>().unwrap();
        let r2 = n21.parse::<u128>().unwrap()..=n22.parse::<u128>().unwrap();
        num_pairs += if r1.start() <= r2.start() && r1.end() >= r2.end() {
            1
        } else if r2.start() <= r1.start() && r2.end() >= r1.end() {
            1
        } else {
            0
        };
    }
    println!("Number of containing pairs: {}", num_pairs);
}

pub fn task2() {
    let mut num_pairs: u128 = 0;
    for pair in get_input(4).lines() {
        let (r1, r2) = pair.split_once(",").unwrap();
        let (n11, n12) = r1.split_once("-").unwrap();
        let (n21, n22) = r2.split_once("-").unwrap();
        let n11: u128 = n11.parse().unwrap();
        let n12: u128 = n12.parse().unwrap();
        let n21: u128 = n21.parse().unwrap();
        let n22: u128 = n22.parse().unwrap();
        num_pairs += if n11 <= n22 && n21 <= n12 { 1 } else { 0 };
    }
    println!("Number of containing pairs: {}", num_pairs);
}
