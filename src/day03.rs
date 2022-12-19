use crate::utils::get_input;

pub fn task1() {
    let mut score: u128 = 0;
    for sack in get_input(3).lines() {
        let (s1, s2) = sack.split_at(sack.len() / 2);
        let dup = s1.chars().filter(|c| s2.contains(*c)).last().unwrap();
        score += if dup.is_lowercase() {
            dup as u8 - b'a' + 1
        } else {
            dup as u8 - b'A' + 27
        } as u128;
    }
    println!("{}", score)
}

pub fn task2() {
    let mut score: u128 = 0;
    for group in get_input(3).lines().collect::<Vec<&str>>().chunks_exact(3) {
        let dup = group[0]
            .chars()
            .filter(|c| group[1].contains(*c) && group[2].contains(*c))
            .last()
            .unwrap();
        score += if dup.is_lowercase() {
            dup as u8 - b'a' + 1
        } else {
            dup as u8 - b'A' + 27
        } as u128;
    }
    println!("{}", score)
}
