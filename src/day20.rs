use crate::utils::get_input;
// This was HEAVILY influenced by https://www.reddit.com/r/adventofcode/comments/zqezkn/comment/j17piu9/
pub fn task1() {
    let nums: Vec<isize> = get_input(20).lines().map(|s| s.parse().unwrap()).collect();
    let mut index: Vec<isize> = (0..(nums.len() as isize)).collect();
    for i in 0..(nums.len() as isize) {
        // get the current index for this original index
        let n = index.iter().position(|&x| x == i).unwrap();
        // Extract it and then put it in the right place
        index.remove(n);
        // put it in the right place! Remember, wrap around works with rem_euclid
        let new_idx = ((n as isize) + nums[i as usize]).rem_euclid(index.len() as isize) as usize;
        // "it" is actually the original index!
        index.insert(new_idx, i);
    }
    // find where 0 is in this new world
    let z_idx = nums.iter().position(|&n| n == 0).unwrap();
    let z = index.iter().position(|&idx| idx == z_idx as isize).unwrap();
    let z: isize = [1000, 2000, 3000]
        .iter()
        .map(|&o| nums[index[(z + o) as usize % index.len()] as usize])
        .sum();
    println!("{}", z);
}

pub fn task2() {
    let nums: Vec<isize> = get_input(20)
        .lines()
        .map(|s| s.parse::<isize>().unwrap() * 811589153)
        .collect();
    let mut index: Vec<isize> = (0..(nums.len() as isize)).collect();

    for i in (0..(nums.len() as isize)).cycle().take(nums.len() * 10) {
        // get the current index for this original index
        let n = index.iter().position(|&x| x == i).unwrap();
        // Extract it and then put it in the right place
        index.remove(n);
        // put it in the right place! Remember, wrap around works with rem_euclid
        let new_idx = ((n as isize) + nums[i as usize]).rem_euclid(index.len() as isize) as usize;
        // "it" is actually the original index!
        index.insert(new_idx, i);
    }
    // find where 0 is in this new world
    let z_idx = nums.iter().position(|&n| n == 0).unwrap();
    let z = index.iter().position(|&idx| idx == z_idx as isize).unwrap();
    let z: isize = [1000, 2000, 3000]
        .iter()
        .map(|&o| nums[index[(z + o) as usize % index.len()] as usize])
        .sum();
    println!("{}", z);
}
