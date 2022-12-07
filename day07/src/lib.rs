use aoclib::parse;
use parse_display::{Display, FromStr};
use std::{
    cell::Cell,
    ops::{Index, IndexMut},
    path::Path,
};

#[derive(Default, Debug, Clone, Copy)]
enum Inode {
    #[default]
    Uninitialized,
    Root,
    Idx(usize),
}

impl Inode {
    fn exists(&self) -> bool {
        !matches!(self, Inode::Uninitialized)
    }
}

impl From<usize> for Inode {
    fn from(idx: usize) -> Self {
        Self::Idx(idx)
    }
}

#[derive(Default, Debug, Clone)]
struct Metadata {
    name: String,
    inode: Inode,
    parent: Inode,
}

#[derive(Default, Debug, Clone, FromStr, Display)]
#[display("dir {metadata.name}")]
#[from_str(default)]
struct Dir {
    metadata: Metadata,
    children: Vec<Inode>,
    size_cache: Cell<Option<u64>>,
}

impl Dir {
    fn size(&self, fs: &Filesystem) -> u64 {
        if let Some(size) = self.size_cache.get() {
            return size;
        }

        let size = self.children.iter().map(|inode| fs[*inode].size(fs)).sum();
        self.size_cache.set(Some(size));
        size
    }
}

#[derive(Default, Debug, Clone, FromStr, Display)]
#[display("{size} {metadata.name}")]
#[from_str(default)]
struct File {
    metadata: Metadata,
    size: u64,
}

/// A terminal line
#[derive(Debug, Clone, FromStr, Display)]
enum Line {
    #[display("{0}")]
    Dir(Dir),
    #[display("{0}")]
    File(File),
    #[display("$ cd {0}")]
    Cd(String),
    #[display("$ ls")]
    Ls,
}

/// A filesystem node
enum Node {
    Dir(Dir),
    File(File),
}

impl From<Dir> for Node {
    fn from(dir: Dir) -> Self {
        Self::Dir(dir)
    }
}

impl From<File> for Node {
    fn from(file: File) -> Self {
        Self::File(file)
    }
}

impl Node {
    fn as_dir(&self) -> Option<&Dir> {
        if let Node::Dir(dir) = self {
            Some(dir)
        } else {
            None
        }
    }

    fn as_dir_mut(&mut self) -> Option<&mut Dir> {
        if let Node::Dir(dir) = self {
            Some(dir)
        } else {
            None
        }
    }

    fn size(&self, fs: &Filesystem) -> u64 {
        match self {
            Node::Dir(dir) => dir.size(fs),
            Node::File(file) => file.size,
        }
    }

    fn metadata(&self) -> &Metadata {
        match self {
            Node::Dir(dir) => &dir.metadata,
            Node::File(file) => &file.metadata,
        }
    }
}

struct Filesystem {
    arena: Vec<Node>,
    root: Node,
}

impl Filesystem {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
            root: Dir::default().into(),
        }
    }

    fn iter(&self) -> impl '_ + Iterator<Item = &Node> {
        std::iter::once(&self.root).chain(self.arena.iter())
    }
}

impl FromIterator<Line> for Filesystem {
    fn from_iter<T: IntoIterator<Item = Line>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let (min, _max) = iter.size_hint();
        let mut fs = Filesystem::with_capacity(min);
        let mut current_dir = Inode::default();

        for line in iter {
            match line {
                Line::Dir(mut dir) => {
                    dir.metadata.parent = current_dir;
                    let inode = fs.arena.len().into();
                    dir.metadata.inode = inode;
                    fs.arena.push(dir.into());
                    fs[current_dir]
                        .as_dir_mut()
                        .expect("current dir is always dir")
                        .children
                        .push(inode);
                }
                Line::File(mut file) => {
                    file.metadata.parent = current_dir;
                    let inode = fs.arena.len().into();
                    file.metadata.inode = inode;
                    fs.arena.push(file.into());
                    fs[current_dir]
                        .as_dir_mut()
                        .expect("current dir is always dir")
                        .children
                        .push(inode);
                }
                Line::Cd(rel_path) => {
                    current_dir = match rel_path.as_str() {
                        "/" => Inode::Root,
                        ".." => {
                            fs[current_dir]
                                .as_dir()
                                .expect("current dir is always dir")
                                .metadata
                                .parent
                        }
                        child_name => fs[current_dir]
                            .as_dir()
                            .expect("current dir is always dir")
                            .children
                            .iter()
                            .copied()
                            .find(|node| {
                                let Some(dir) = fs[*node].as_dir() else { return false };
                                dir.metadata.name == child_name
                            })
                            .unwrap_or_else(|| {
                                eprintln!(
                                    "attempting to cd to unknown child directory: {child_name}"
                                );
                                Inode::default()
                            }),
                    };
                }
                Line::Ls => {
                    // no op
                }
            }
        }

        fs
    }
}

impl Index<Inode> for Filesystem {
    type Output = Node;

    fn index(&self, index: Inode) -> &Self::Output {
        match index {
            Inode::Uninitialized => panic!("attempted to use uninitialized inode as index"),
            Inode::Root => &self.root,
            Inode::Idx(idx) => &self.arena[idx],
        }
    }
}

impl IndexMut<Inode> for Filesystem {
    fn index_mut(&mut self, index: Inode) -> &mut Self::Output {
        match index {
            Inode::Uninitialized => panic!("attempted to use uninitialized inode as index"),
            Inode::Root => &mut self.root,
            Inode::Idx(idx) => &mut self.arena[idx],
        }
    }
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let fs: Filesystem = parse(input)?.collect();
    let size_sum: u64 = fs
        .iter()
        .filter_map(|node| {
            node.as_dir().and_then(|dir| {
                let size = dir.size(&fs);
                (size < 100_000).then_some(size)
            })
        })
        .sum();
    println!("size sum: {size_sum}");
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let fs: Filesystem = parse(input)?.collect();

    let total_disk_space = 70_000_000;
    let need_unused_space: u64 = 30_000_000;
    let used_space = fs.root.size(&fs);
    let unused_space = total_disk_space - used_space;
    let need_to_clear = need_unused_space.checked_sub(unused_space);

    match need_to_clear {
        None => println!("pt. 2: don't need to clear any directories"),
        Some(need_to_clear) => {
            let smallest_deleteable_directory = fs
                .iter()
                .filter_map(|node| node.as_dir())
                .filter(|dir| dir.size(&fs) >= need_to_clear)
                .min_by_key(|dir| dir.size(&fs))
                .expect(
                    "at least one directory is big enough that deleting it clears enough space",
                );
            let path = {
                let mut path = Vec::new();
                let mut current_dir = smallest_deleteable_directory.metadata.inode;
                while current_dir.exists() {
                    let metadata = fs[current_dir].metadata();
                    path.push(metadata.name.as_str());
                    current_dir = metadata.parent;
                }
                path.reverse();
                path.join("/")
            };
            let size = smallest_deleteable_directory.size(&fs);
            println!("pt. 2: deleting {path} clearing {size}");
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("no solution found")]
    NoSolution,
}
