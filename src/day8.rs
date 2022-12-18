use crate::utils::get_input;

pub fn task1() {
    let mut rows: Vec<Vec<u8>> = vec![vec![]];
    let mut cols: Vec<Vec<u8>> = vec![];
    let mut trees: Vec<u8> = vec![];

    let inp = get_input(8);
    let mut lines = inp.lines();

    for h in lines.next().unwrap().chars() {
        let h = h.to_digit(10).unwrap() as u8;
        rows[0].push(h);
        cols.push(vec![h]);
        trees.push(h);
    }

    for line in lines {
        let mut row = vec![];
        for (c, h) in line.chars().enumerate() {
            let h = h.to_digit(10).unwrap() as u8;
            row.push(h);
            cols[c].push(h);
            trees.push(h);
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }

    let count = trees
        .iter()
        .enumerate()
        .map(|(i, t)| (t, (i / cols.len(), i % cols.len())))
        .filter(|&(&tree, (r, c))| {
            let (left, right) = rows[r].split_at(c);
            let (above, below) = cols[c].split_at(r);
            [
                left.iter().skip(0),
                right.iter().skip(1),
                above.iter().skip(0),
                below.iter().skip(1),
            ]
            .iter_mut()
            .any(|d| d.all(|&t| t < tree))
        })
        .count();
    println!("{}", count);
}

fn scene_score(tree: u8, directions: &mut [impl Iterator<Item = u8>]) -> usize {
    directions
        .iter_mut()
        .map(|dir| {
            let mut was_stopped = false;

            let trees = dir
                .take_while(|&t| {
                    if t >= tree {
                        was_stopped = true;
                        false
                    } else {
                        true
                    }
                })
                .count();
            if was_stopped {
                trees + 1
            } else {
                trees
            }
        })
        .product()
}

pub fn task2() {
    let mut rows: Vec<Vec<u8>> = vec![vec![]];
    let mut cols: Vec<Vec<u8>> = vec![];
    let mut trees: Vec<u8> = vec![];

    let inp = get_input(8);
    let mut lines = inp.lines();

    for h in lines.next().unwrap().chars() {
        let h = h.to_digit(10).unwrap() as u8;
        rows[0].push(h);
        cols.push(vec![h]);
        trees.push(h);
    }

    for line in lines {
        let mut row = vec![];
        for (c, h) in line.chars().enumerate() {
            let h = h.to_digit(10).unwrap() as u8;
            row.push(h);
            cols[c].push(h);
            trees.push(h);
        }
        if !row.is_empty() {
            rows.push(row);
        }
    }

    let high_score = trees
        .iter()
        .enumerate()
        .map(|(i, t)| (t, (i / cols.len(), i % cols.len())))
        .map(|(&tree, (r, c))| {
            let (left, right) = rows[r].split_at(c);
            let (above, below) = cols[c].split_at(r);
            let mut left = left.to_vec();
            let mut above = above.to_vec();
            left.reverse();
            above.reverse();

            scene_score(
                tree,
                &mut [
                    left.iter().cloned(),
                    above.iter().cloned(),
                    right[1..].iter().cloned(),
                    below[1..].iter().cloned(),
                ],
            )
        })
        .max()
        .unwrap();
    println!("{}", high_score);
}
