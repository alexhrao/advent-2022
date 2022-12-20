use pathfinding::matrix::Matrix;
use pathfinding::prelude::bfs;

use crate::utils::get_input;

pub fn task1() {
    let mut graph = Matrix::from_rows(get_input(12).lines().map(|l| l.bytes())).unwrap();
    let start = graph.values().position(|&e| e == 'S' as u8).unwrap();
    let start = (start / graph.columns, start % graph.columns);
    let end = graph.values().position(|&e| e == 'E' as u8).unwrap();
    let end = (end / graph.columns, end % graph.columns);

    graph[start] = 'a' as u8;
    graph[end] = 'z' as u8;

    let stuff = bfs(
        &start,
        |&(r, c)| {
            [
                (r.saturating_sub(1), c),
                (r.saturating_add(1), c),
                (r, c.saturating_sub(1)),
                (r, c.saturating_add(1)),
            ]
            .iter()
            .filter(|&&rc| graph.within_bounds(rc))
            .filter(|&&rc| graph[rc] <= (graph[(r, c)] + 1))
            .copied()
            .collect::<Vec<(usize, usize)>>()
        },
        |&rc| rc == end,
    );

    println!("{}", stuff.unwrap().len() - 1);
}

pub fn task2() {
    let mut graph = Matrix::from_rows(get_input(12).lines().map(|l| l.bytes())).unwrap();
    let start = graph.values().position(|&e| e == 'S' as u8).unwrap();
    let start = (start / graph.columns, start % graph.columns);
    let end = graph.values().position(|&e| e == 'E' as u8).unwrap();
    let end = (end / graph.columns, end % graph.columns);

    graph[start] = 'a' as u8;
    graph[end] = 'z' as u8;

    let path = graph
        .values()
        .enumerate()
        .filter(|&i| *i.1 == 'a' as u8)
        .map(|(i, _)| (i / graph.columns, i % graph.columns))
        .filter(|&(r, c)| {
            [
                (r.saturating_sub(1), c),
                (r.saturating_add(1), c),
                (r, c.saturating_sub(1)),
                (r, c.saturating_add(1)),
            ]
            .iter()
            .filter(|&&rc| graph.within_bounds(rc))
            .any(|&rc| graph[rc] != 'a' as u8)
        })
        .filter_map(|start| {
            bfs(
                &start,
                |&(r, c)| {
                    [
                        (r.saturating_sub(1), c),
                        (r.saturating_add(1), c),
                        (r, c.saturating_sub(1)),
                        (r, c.saturating_add(1)),
                    ]
                    .iter()
                    .filter(|&&rc| graph.within_bounds(rc))
                    .filter(|&&rc| graph[rc] <= (graph[(r, c)] + 1))
                    .copied()
                    .collect::<Vec<(usize, usize)>>()
                },
                |&rc| rc == end,
            )
        })
        .min_by_key(|path| path.len())
        .unwrap();
    // TOO HIGH
    println!("{}", path.len() - 1);
}
