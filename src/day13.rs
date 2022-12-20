use crate::utils::get_input;
use serde_json::Value;

fn compare(left: &Value, right: &Value) -> Option<bool> {
    if let Some((left, right)) = left.as_i64().zip(right.as_i64()) {
        if left != right {
            return Some(left < right);
        }
    } else if let Some((left, right)) = left.as_array().zip(right.as_array()) {
        for i in 0..left.len().min(right.len()) {
            let res = compare(&left[i], &right[i]);
            if res.is_some() {
                return res;
            }
        }
        if left.len() != right.len() {
            return Some(left.len() < right.len());
        }
    } else if left.is_array() {
        return compare(left, &Value::Array(vec![right.clone()]));
    } else if right.is_array() {
        return compare(&Value::Array(vec![left.clone()]), right);
    }
    None
}

pub fn task1() {
    let pairs: Vec<(Value, Value)> = get_input(13)
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|pair| {
            (
                serde_json::from_str(pair[0]).unwrap(),
                serde_json::from_str(pair[1]).unwrap(),
            )
        })
        .collect();
    let mut sum = 0;
    for (idx, packets) in pairs.iter().enumerate() {
        let (left, right) = packets;
        if let Some(comparison) = compare(left, right) {
            sum += if comparison { idx + 1 } else { 0 };
        }
    }
    println!("Sum: {}", sum);
}

pub fn task2() {
    let dividers = vec!["[[2]]", "[[6]]"];
    let mut packets: Vec<Value> = get_input(13)
        .lines()
        .filter(|s| !s.is_empty())
        .map(serde_json::from_str)
        .map(Result::unwrap)
        .collect();
    packets.extend(dividers.iter().map(|s| serde_json::from_str(s).unwrap()));
    let mut packets: Vec<(usize, Value)> = packets.into_iter().enumerate().collect();
    packets.sort_by(|(_, p1), (_, p2)| match compare(p1, p2) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });

    let idx1 = packets.iter().position(|&(i, _)| i == packets.len() - 1).unwrap() + 1;
    let idx2 = packets.iter().position(|&(i, _)| i == packets.len() - 2).unwrap() + 1;
    println!("{}", idx1 * idx2);
}
