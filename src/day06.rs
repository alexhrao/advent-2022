use std::collections::HashSet;

use crate::utils::get_input;

pub fn task1() {
    for (i, w) in get_input(6).as_bytes().windows(4).enumerate() {
        if i < 3 {
            continue;
        }
        let mut a = HashSet::new();
        if w.iter().into_iter().all(move |x| a.insert(*x)) {
            println!("{}", 4 + i);
            return;
        }
    }
    panic!()
}

pub fn task2() {
    for (i, w) in get_input(6).as_bytes().windows(14).enumerate() {
        if i < 13 {
            continue;
        }
        let mut a = HashSet::new();
        if w.iter().into_iter().all(move |x| a.insert(*x)) {
            println!("{}", 14 + i);
            return;
        }
    }
    panic!()
}
