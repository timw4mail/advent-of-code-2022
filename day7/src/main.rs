#![allow(dead_code)]

use std::collections::HashMap;

const MAX_DIR_SIZE: u128 = 100_000;
const TOTAL_DISK_SPACE: u128 = 70_000_000;
const MIN_SPACE_REQUIRED: u128 = 30_000_000;

#[derive(Debug)]
struct File {
    name: String,
    size: u128,
}

impl File {
    fn new<T: ToString + ?Sized>(size: &T, name: &T) -> Self {
        let size = size.to_string().parse::<u128>().unwrap();
        let name = name.to_string();

        File { name, size }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Dir {
    parent: String,
    name: String,
    files: Vec<File>,
    subdirs: Vec<String>,
}

impl Dir {
    fn new<T: ToString + ?Sized>(parent: &T, name: &T) -> Self {
        Dir {
            parent: parent.to_string(),
            name: name.to_string(),
            files: Vec::new(),
            subdirs: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_subdir(&mut self, path: String) {
        self.subdirs.push(path);
    }

    fn get_loose_files_size(&self) -> u128 {
        self.files
            .iter()
            .map(|file| file.size)
            .reduce(|accum, item| accum + item)
            .unwrap_or(0)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
enum LineType {
    Cd(String),
    Ls,
    Dir(String),
    FileAndSize(String, String),
}

use LineType::*;

impl LineType {
    fn from(line: &str) -> LineType {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();

        match parts[0] {
            "$" => match parts[1] {
                "cd" => Cd(parts[2].to_string()),
                "ls" => Ls,
                _ => panic!("Invalid command"),
            },
            "dir" => Dir(parts[1].to_string()),
            _ => FileAndSize(parts[0].to_string(), parts[1].to_string()),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct DirMap {
    current_path: String,
    map: HashMap<String, Dir>,
}

impl DirMap {
    fn new() -> Self {
        let current_path = "/".to_string();
        let mut map: HashMap<String, Dir> = HashMap::new();
        map.insert(current_path.clone(), Dir::new("", ""));

        DirMap { current_path, map }
    }

    fn cd<T: ToString + ?Sized>(&mut self, new_dir: &T) {
        let current_path = self.current_path.clone();
        let new = new_dir.to_string();

        match new.as_str() {
            "/" => {
                self.current_path = new.to_string();
            }
            ".." => {
                let mut dir_parts: Vec<&str> = current_path.split('/').collect();
                let _ = dir_parts.pop();
                self.current_path = dir_parts.join("/");
            }
            _ => {
                self.current_path.push('/');
                self.current_path.push_str(&new);
            }
        }
    }

    fn dir<T: ToString + ?Sized>(&mut self, dir: &T) {
        let parent = self.current_path.clone();
        let name = dir.to_string();

        let mut full_path = parent.clone();
        full_path.push('/');
        full_path.push_str(&name);

        // Add the new Dir to the path map
        if !self.map.contains_key(&full_path) {
            self.map.insert(full_path.clone(), Dir::new(&parent, &name));
        }

        // Add the new Dir to the list of subdirectories to the Dir mapped to the current path
        self.map
            .get_mut(&self.current_path)
            .expect(&format!(
                "This dir ({}) should already exist",
                &self.current_path
            ))
            .add_subdir(full_path.clone());
    }

    fn parse(&mut self, item: LineType) {
        match item {
            Cd(s) => self.cd(&s),
            Ls => {}
            Dir(s) => self.dir(&s),
            FileAndSize(size, name) => {
                self.map
                    .get_mut(&self.current_path)
                    .expect(&format!(
                        "This dir ({}) should already exist",
                        &self.current_path
                    ))
                    .add_file(File::new(&size, &name));
            }
        }
    }
}

// ----------------------------------------------------------------------------

fn get_path_size_map(dir_map: &DirMap) -> HashMap<String, u128> {
    let mut size_map: HashMap<String, u128> = HashMap::new();

    // Get the sizes of the leaf node directories
    dir_map
        .map
        .iter()
        .filter(|(_, v)| v.subdirs.len() == 0)
        .for_each(|(k, v)| {
            size_map.insert(k.to_string(), v.get_loose_files_size());
        });

    // Calculate dir sizes by the length of the path from largest to smallest,
    // so we can start with the lowest branches of the tree when calculating folder sizes
    let mut branch_paths: Vec<&String> = dir_map
        .map
        .iter()
        .filter(|(_, v)| v.subdirs.len() > 0)
        .map(|(k, _)| k)
        .collect();
    branch_paths.sort();
    branch_paths.reverse();

    branch_paths.into_iter().for_each(|path| {
        let dir = dir_map.map.get(path).unwrap();

        let base_size = dir.get_loose_files_size();
        let subdir_size: u128 = dir
            .subdirs
            .iter()
            .map(|sub| {
                *size_map
                    .get(sub)
                    .expect("Dir {} should already have had its size calculated")
            })
            .reduce(|accum, item| accum + item)
            .unwrap_or(0);

        size_map.insert(path.to_string(), base_size + subdir_size);
    });

    size_map
}

fn calculate_needed_space(used_space: u128) -> u128 {
    MIN_SPACE_REQUIRED - (TOTAL_DISK_SPACE - used_space)
}

fn find_size_of_dir(min_size: u128, size_map: &HashMap<String, u128>) -> u128 {
    size_map
        .iter()
        .map(|(_, v)| *v)
        .filter(|v| *v > min_size)
        .min()
        .unwrap()
}

fn calculate_sum_of_dirs(size_map: &HashMap<String, u128>) -> u128 {
    size_map
        .iter()
        .filter(|(_, v)| **v < MAX_DIR_SIZE)
        .fold(0u128, |acc, (_, v)| acc + *v)
}

fn main() {
    let file_str = include_str!("input.txt");
    let mut path_map = DirMap::new();

    file_str
        .lines()
        .map(|line| LineType::from(line))
        .for_each(|cmd| path_map.parse(cmd));

    let size_map = get_path_size_map(&path_map);
    let size_sum = calculate_sum_of_dirs(&size_map);

    let used_space = *size_map.get("/").unwrap();
    let smallest_dir = find_size_of_dir(calculate_needed_space(used_space), &size_map);

    println!("Part 1: Sum of dirs 100K or smaller {:#?}", size_sum);
    println!("Part 2: Size of smallest dir to delete: {}", smallest_dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_size_of_dir() {
        let mut size_map: HashMap<String, u128> = HashMap::new();
        size_map.insert("//e".to_string(), 584);
        size_map.insert("//a".to_string(), 94853);
        size_map.insert("//d".to_string(), 24933642);
        size_map.insert("/".to_string(), 48381165);

        let res = find_size_of_dir(8381165, &size_map);

        assert_eq!(res, 24933642);
    }

    #[test]
    fn test_calculate_needed_space() {
        let res = calculate_needed_space(48381165);

        assert_eq!(res, 8381165);
    }
}
