use std::collections::HashMap;

use regex::Regex;

use crate::utils::get_input;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Div,
            _ => panic!(),
        }
    }
}

impl Operation {
    pub fn execute(&self, left: i128, right: i128) -> i128 {
        match self {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => left / right,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Div => Operation::Mul,
            Operation::Mul => Operation::Div,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Monkey {
    id: String,
    dependents: Option<(String, String)>,
    op: Option<Operation>,
    number: Option<i128>,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MonkeyResult {
    Known(i128),
    Unknown(Vec<String>),
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum MonkeyOperation {
    Number(i128),
    Monkey(String),
}

impl MonkeyOperation {
    pub fn get_known(left: &MonkeyOperation, right: &MonkeyOperation) -> i128 {
        if let MonkeyOperation::Number(num) = left {
            *num
        } else if let MonkeyOperation::Number(num) = right {
            *num
        } else {
            panic!()
        }
    }
}

impl Monkey {
    pub fn evaluate(&self, monkeys: &HashMap<String, Monkey>) -> i128 {
        if let Some(num) = self.number {
            num
        } else if let Some((m1, m2)) = &self.dependents {
            if let Some(op) = self.op {
                op.execute(monkeys[m1].evaluate(monkeys), monkeys[m2].evaluate(monkeys))
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }

    pub fn evaluate_for_equality(&self, monkeys: &HashMap<String, Monkey>) -> MonkeyResult {
        if self.id == "humn" {
            MonkeyResult::Unknown(vec![])
        } else if let Some(num) = self.number {
            MonkeyResult::Known(num)
        } else if let Some((m1, m2)) = &self.dependents {
            if let Some(op) = self.op {
                let left = monkeys[m1].evaluate_for_equality(monkeys);
                let right = monkeys[m2].evaluate_for_equality(monkeys);
                let lr = (left, right);
                if let (MonkeyResult::Known(l), MonkeyResult::Known(r)) = lr {
                    MonkeyResult::Known(op.execute(l, r))
                } else if let (_, MonkeyResult::Unknown(mut past)) = lr {
                    past.push(self.id.clone());
                    MonkeyResult::Unknown(past)
                } else if let (MonkeyResult::Unknown(mut past), _) = lr {
                    past.push(self.id.clone());
                    MonkeyResult::Unknown(past)
                } else {
                    panic!()
                }
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}

pub fn task1() {
    let num_re = Regex::new(r"(^\w{4}): (\d+)$").unwrap();
    let op_re = Regex::new(r"^(\w{4}): (\w{4}) ([+\-*/]) (\w{4})$").unwrap();
    let monkeys: HashMap<String, Monkey> = get_input(21)
        .lines()
        .map(|m| {
            let monkey = if let Some(caps) = num_re.captures(m) {
                Monkey {
                    id: caps[1].to_string(),
                    dependents: None,
                    op: None,
                    number: Some(caps[2].parse().unwrap()),
                }
            } else if let Some(caps) = op_re.captures(m) {
                Monkey {
                    id: caps[1].to_string(),
                    dependents: Some((caps[2].to_string(), caps[4].to_string())),
                    op: Some(caps[3].into()),
                    number: None,
                }
            } else {
                panic!()
            };
            (monkey.id.clone(), monkey)
        })
        .collect();
    println!("{}", monkeys["root"].evaluate(&monkeys));
}

pub fn task2() {
    let num_re = Regex::new(r"(^\w{4}): (\d+)$").unwrap();
    let op_re = Regex::new(r"^(\w{4}): (\w{4}) ([+\-*/]) (\w{4})$").unwrap();
    let monkeys: HashMap<String, Monkey> = get_input(21)
        .lines()
        .map(|m| {
            let mut monkey = if let Some(caps) = num_re.captures(m) {
                Monkey {
                    id: caps[1].to_string(),
                    dependents: None,
                    op: None,
                    number: Some(caps[2].parse().unwrap()),
                }
            } else if let Some(caps) = op_re.captures(m) {
                Monkey {
                    id: caps[1].to_string(),
                    dependents: Some((caps[2].to_string(), caps[4].to_string())),
                    op: Some(caps[3].into()),
                    number: None,
                }
            } else {
                panic!()
            };
            if monkey.id == "humn" {
                monkey.number = None
            }
            (monkey.id.clone(), monkey)
        })
        .collect();
    if let MonkeyResult::Unknown(past) = monkeys["root"].evaluate_for_equality(&monkeys) {
        // for each past, get a tuple
        let mut past: Vec<_> = past
            .into_iter()
            .map(|m_id| {
                let monkey = &monkeys[&m_id];
                if let Some(deps) = monkey.dependents.clone() {
                    if let MonkeyResult::Known(left) =
                        monkeys[&deps.0].evaluate_for_equality(&monkeys)
                    {
                        (
                            MonkeyOperation::Number(left),
                            monkey.op.unwrap(),
                            MonkeyOperation::Monkey(deps.1),
                        )
                    } else if let MonkeyResult::Known(right) =
                        monkeys[&deps.1].evaluate_for_equality(&monkeys)
                    {
                        (
                            MonkeyOperation::Monkey(deps.0),
                            monkey.op.unwrap(),
                            MonkeyOperation::Number(right),
                        )
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            })
            .collect();
        let (root_left, _, root_right) = past.pop().unwrap();
        let mut seed = MonkeyOperation::get_known(&root_left, &root_right);

        for (left, op, right) in past.into_iter().rev() {
            if let MonkeyOperation::Number(r) = right {
                seed = op.opposite().execute(seed, r);
            } else if let MonkeyOperation::Number(l) = left {
                if op == Operation::Add || op == Operation::Mul {
                    seed = op.opposite().execute(seed, l);
                } else {
                    seed = op.execute(l, seed)
                }
            }
        }
        println!("{}", seed);
    }
}
