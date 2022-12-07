use std::collections::HashMap;

const MAX_DIR_SIZE: u128 = 100_000;

#[derive(Debug)]
struct File {
    name: String,
    size: u128,
}

impl File {
    fn new<T: ToString + ?Sized>(raw_line: &T) -> Self {
        let raw = raw_line.to_string();
        let parts: Vec<&str> = raw.split(' ').collect();
        let size = parts[0].parse::<u128>().unwrap();
        let name = parts[1].to_string();

        File { name, size }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug)]
struct Dir {
    parent: String,
    name: String,
    files: Vec<File>,
    dirs: Vec<String>,
}

impl Dir {
    fn new<T: ToString + ?Sized>(parent: &T, name: &T) -> Self {
        Dir {
            parent: parent.to_string(),
            name: name.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }

    fn get_loose_files_size(&self) -> u128 {
        self.files
            .iter()
            .map(|file| file.size)
            .reduce(|accum, item| accum + item)
            .unwrap()
    }
}

// ----------------------------------------------------------------------------

struct DirMap {
    current_path: String,
    map: HashMap<String, Dir>,
}

impl DirMap {
    fn new() -> Self {
        DirMap {
            current_path: "/".to_string(),
            map: HashMap::new(),
        }
    }
}

// ----------------------------------------------------------------------------

fn main() {
    let file_str = include_str!("input.txt");
    let path_map = DirMap::new();
}
