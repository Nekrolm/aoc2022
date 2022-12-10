use std::{
    collections::{hash_map::Entry},
    io::BufRead,
};

type DirInode = usize;

#[derive(Debug)]
enum Node {
    File { size : usize },
    Directory(DirInode),
}


#[derive(Debug)]
struct Directory {
    items: std::collections::HashMap<String, Node>,
    parent: DirInode,
}


#[derive(Debug)]
struct FS {
    directories: Vec<Directory>,
}

impl FS {
    fn new() -> Self {
        let root = Directory {
            items: Default::default(),
            parent: 0,
        };
        let directories = vec![root];
        Self { directories }
    }

    fn root(&self) -> DirInode {
        0
    }

    fn cd(&self, dir: DirInode, fname: &str) -> Option<DirInode> {
        match fname {
            "/" => Some(self.root()),
            ".." => Some(self.directories[dir].parent),
            name => self.directories[dir]
                .items
                .get(name)
                .and_then(|node| match node {
                    Node::File {..} => None,
                    Node::Directory(dir) => Some(*dir),
                }),
        }
    }

    fn add_file(&mut self, dir: DirInode, fname: String, size: usize) {
        self.directories[dir].items.entry(fname).or_insert_with(|| {
            Node::File {size}
        });
    }

    fn touch_directory(&mut self, dir: DirInode, dname: String) {
        let new_node = self.directories.len();
        let newdir = match self.directories[dir].items.entry(dname) {
            Entry::Occupied(_) => None,
            Entry::Vacant(v) => {
                v.insert(Node::Directory(new_node));
                Some(Directory {
                    items: Default::default(),
                    parent: dir,
                })
            }
        };
        self.directories.extend(newdir);
    }

    fn directory_sizes(&self) -> Vec<usize> {
        let mut sizes = Vec::with_capacity(self.directories.len());
        sizes.resize(self.directories.len(), 0);
        self.directories.iter().enumerate().rev().for_each(|(dirno, dir)| {
            let dir_size: usize = dir.items.values().map(|item| match item {
                &Node::File { size } => size,
                &Node::Directory(inode) => sizes[inode]
            }).sum();
            sizes[dirno] = dir_size;
        });
        sizes
    }
}

enum Command {
    Ls,
    Cd { dname: String },
    AddFile { name: String, size: usize },
    TouchDir { dname: String },
}

fn parse_command(input: &str) -> Command {
    let mut tokens = input.split_whitespace();
    let first = tokens.next().expect("at least two token expected");
    let second = tokens.next().expect("at least two tokens expected");
    match (first, second) {
        ("$", "ls") => Command::Ls,
        ("$", "cd") => Command::Cd {
            dname: tokens.next().expect("dirname expected").to_string(),
        },
        ("dir", name) => Command::TouchDir {
            dname: name.to_string(),
        },
        (size, name) => Command::AddFile {
            name: name.to_string(),
            size: size.parse().expect("should be int"),
        },
    }
}

fn main() {
    let infile = aoc2022::get_input_file();
    let parser = std::io::BufReader::new(infile);
    let mut fs = FS::new();
    let mut dirno = fs.root();

    parser
        .lines()
        .map(Result::unwrap)
        .filter(|s| !s.is_empty())
        .map(|line| parse_command(&line))
        .for_each(|cmd|{
            match cmd {
                Command::Ls => (),
                Command::Cd { dname } => {
                    let newdir = fs.cd(dirno, &dname).expect("should be existing dir");
                    dirno = newdir
                }
                Command::AddFile { name, size } => fs.add_file(dirno, name, size),
                Command::TouchDir { dname } => fs.touch_directory(dirno, dname)
            }
        });

    println!("{fs:#?}");

    let sizes = fs.directory_sizes();
    let used = sizes[0];
    let total = 70000000;
    let current_empty = total - used;
    let need_empty = 30000000;



    // let size : usize = sizes.into_iter().filter(|&size| size <= 100000).sum();

    let size = sizes.into_iter().filter_map(|size| {
        (current_empty + size >= need_empty).then_some(size)
    }).min();

    println!("{size:?}")
}
