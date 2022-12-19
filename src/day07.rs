use std::collections::HashMap;

use regex::Regex;

use crate::utils::get_input;

struct File {
    pub size: u128,
}
struct Directory {
    pub inode: usize,
    pub parent: Option<usize>,
    pub dirs: HashMap<String, usize>,
    pub files: HashMap<String, File>,
}

impl Directory {
    pub fn new(inode: usize) -> Directory {
        Directory {
            inode,
            parent: None,
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
    pub fn with_parent(inode: usize, parent: usize) -> Directory {
        Directory {
            inode,
            parent: Some(parent),
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }
    pub fn size(&self, directories: &Vec<Directory>) -> u128 {
        self.files.values().map(|f| f.size).sum::<u128>()
            + self
                .dirs
                .values()
                .map(|d| directories[*d].size(directories))
                .sum::<u128>()
    }
}

pub fn task1() {
    let mut directories: Vec<Directory> = vec![];
    let root = Directory::new(directories.len());
    let mut dir: usize = 0;

    directories.push(root);
    for line in get_input(7).lines() {
        let cd_re = Regex::new(r"^\$ cd (.+)$").unwrap();
        let dir_re = Regex::new(r"^dir (\w+)$").unwrap();
        let file_re = Regex::new(r"^(\d+) (.+)$").unwrap();
        if let Some(caps) = cd_re.captures(line) {
            if &caps[1] == ".." {
                dir = directories[dir].parent.unwrap();
            } else if &caps[1] == "/" {
                dir = 0;
            } else {
                dir = directories[dir].dirs[&caps[1]];
            }
        } else if let Some(caps) = dir_re.captures(line) {
            let child = Directory::with_parent(directories.len(), dir);
            directories[dir]
                .dirs
                .insert(caps[1].to_string(), child.inode);
            directories.push(child);
        } else if let Some(caps) = file_re.captures(line) {
            let child = File {
                size: caps[1].parse().unwrap(),
            };
            directories[dir].files.insert(caps[2].to_string(), child);
        }
    }
    println!(
        "{}",
        directories
            .iter()
            .map(|d| d.size(&directories))
            .filter(|s| *s <= 100000)
            .sum::<u128>()
    );
}

pub fn task2() {
    let mut directories: Vec<Directory> = vec![];
    let root = Directory::new(directories.len());
    let mut dir: usize = 0;

    directories.push(root);
    for line in get_input(7).lines() {
        let cd_re = Regex::new(r"^\$ cd (.+)$").unwrap();
        let dir_re = Regex::new(r"^dir (\w+)$").unwrap();
        let file_re = Regex::new(r"^(\d+) (.+)$").unwrap();
        if let Some(caps) = cd_re.captures(line) {
            if &caps[1] == ".." {
                dir = directories[dir].parent.unwrap();
            } else if &caps[1] == "/" {
                dir = 0;
            } else {
                dir = directories[dir].dirs[&caps[1]];
            }
        } else if let Some(caps) = dir_re.captures(line) {
            let child = Directory::with_parent(directories.len(), dir);
            directories[dir]
                .dirs
                .insert(caps[1].to_string(), child.inode);
            directories.push(child);
        } else if let Some(caps) = file_re.captures(line) {
            let child = File {
                size: caps[1].parse().unwrap(),
            };
            directories[dir].files.insert(caps[2].to_string(), child);
        }
    }
    let mut sizes = directories.iter().map(|d| d.size(&directories));

    let free_space = 70000000 - sizes.next().unwrap();

    println!(
        "{}",
        sizes
            .filter(|&s| s >= (30000000 - free_space))
            .min()
            .unwrap()
    );
}
