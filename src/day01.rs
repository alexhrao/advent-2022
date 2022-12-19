use crate::utils::get_input;

pub fn task1() {
    let mut elves: Vec<u128> = vec![0];
    for a in get_input(1).lines() {
        if a.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += a.parse::<u128>().unwrap();
        }
    }
    println!("Max: {}", elves.iter().max().unwrap());
}

pub fn task2() {
    let mut elves: Vec<u128> = vec![0];
    for a in get_input(1).lines() {
        if a.is_empty() {
            elves.push(0);
        } else {
            *elves.last_mut().unwrap() += a.parse::<u128>().unwrap();
        }
    }
    let max1 = *elves.iter().max().unwrap();
    let idx1 = elves.iter().position(|e| *e == max1).unwrap();
    elves.remove(idx1);
    let max2 = *elves.iter().max().unwrap();
    let idx2 = elves.iter().position(|e| *e == max2).unwrap();
    elves.remove(idx2);
    let max3 = *elves.iter().max().unwrap();
    println!("{}", max1 + max2 + max3)
}
