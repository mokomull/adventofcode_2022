use prelude::log::debug;
use prelude::*;
use wasm_bindgen::JsValue;

#[derive(Debug)]
enum Command {
    Cd(Component),
    Ls(Vec<InputEntry>),
}

#[derive(Debug)]
enum Component {
    Root,
    Parent,
    Path(String),
}

#[derive(Debug)]
enum InputEntry {
    File { size: u32, name: String },
    Directory(String),
}

use Command::*;
use Component::*;

pub struct Solution {
    commands: Vec<Command>,
}

impl Solution {
    pub fn new(input: &str) -> Self {
        init();

        let mut commands = vec![];
        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            let command = match line {
                "$ cd /" => Cd(Root),
                "$ cd .." => Cd(Parent),
                "$ ls" => {
                    let mut entries = vec![];
                    while let Some(entry) = lines.next_if(|&x| !x.starts_with("$")) {
                        if entry.starts_with("dir ") {
                            entries.push(InputEntry::Directory(
                                entry.strip_prefix("dir ").unwrap().to_owned(),
                            ))
                        } else {
                            let (size, name) =
                                entry.split_once(' ').expect("there should be a space");
                            entries.push(InputEntry::File {
                                size: size.parse().expect("couldn't parse size"),
                                name: name.to_owned(),
                            })
                        }
                    }
                    Ls(entries)
                }
                x if x.starts_with("$ cd ") => {
                    Cd(Path(x.strip_prefix("$ cd ").unwrap().to_owned()))
                }
                x => panic!("unparsed line {:?}", x),
            };

            commands.push(command);
        }

        debug! {"parsed: {:#?}", commands}

        Self { commands }
    }

    pub fn part1(&self) -> Result<u64, JsValue> {
        let tree = build_tree(self.commands.iter())?;
        debug!("{:#?}", tree);

        let mut sum = 0;

        recurse_through(&tree, &mut |this_level| {
            // part 1's condition seems to allow subtrees to be double-counted, so
            if this_level <= 100_000 {
                sum += this_level;
            }
        });

        Ok(sum)
    }

    pub fn part2(&self) -> Result<u64, JsValue> {
        let tree = build_tree(self.commands.iter())?;

        let total = recurse_through(&tree, &mut |_| ());
        debug!("total size is {}", total);
        assert!(total > 40000000);
        let needed = total - 40000000;
        debug!("needed: {}", needed);

        let mut smallest_directory_that_is_big_enough = u64::MAX;
        recurse_through(&tree, &mut |this_level| {
            if this_level >= needed {
                smallest_directory_that_is_big_enough =
                    std::cmp::min(smallest_directory_that_is_big_enough, this_level);
            }
        });

        Ok(smallest_directory_that_is_big_enough)
    }
}

type Tree<'a> = HashMap<&'a str, TreeEntry<'a>>;

#[derive(Debug)]
enum TreeEntry<'a> {
    File(u32),
    Directory(Tree<'a>),
}

enum UpwardCd {
    Parent,
    Root,
}

fn build_tree<'a, I>(commands: I) -> Result<Tree<'a>, JsValue>
where
    I: Iterator<Item = &'a Command>,
{
    let mut tree = HashMap::new();
    let mut it = commands.peekable();

    while it.peek().is_some() {
        // whether it was `cd ..` or `cd /` doesn't matter here, since we've already traversed all
        // the way back to the root...
        build_level(&mut it, &mut tree)?;
    }

    Ok(tree)
}

fn build_level<'a, I>(commands: &mut I, tree: &mut Tree<'a>) -> Result<UpwardCd, JsValue>
where
    I: Iterator<Item = &'a Command>,
{
    while let Some(command) = commands.next() {
        match command {
            Cd(Parent) => return Ok(UpwardCd::Parent),
            Cd(Root) => return Ok(UpwardCd::Root),
            Cd(Path(p)) => {
                let subtree = tree.get_mut(p.as_str()).ok_or_else(|| {
                    format!("tried to chdir to {:?} before it was seen by an ls", p)
                })?;
                match subtree {
                    TreeEntry::Directory(t) => {
                        // recursively build the next commands into this tree
                        match build_level(commands, t)? {
                            // if we got back here from a `cd ..`, then swallow it and keep
                            // iterating through commands and their responses
                            UpwardCd::Parent => (),
                            // but if we got back because somewhere deep in the tree did a `cd /`,
                            // then return all the way back up to the root build_tree().
                            UpwardCd::Root => return Ok(UpwardCd::Root),
                        }
                    }
                    TreeEntry::File(_) => {
                        return Err(format!("tried to chdir into a file: {:?}", p).into())
                    }
                }
            }
            Ls(entries) => {
                for entry in entries {
                    match entry {
                        InputEntry::File { size, name } => {
                            tree.insert(name.as_str(), TreeEntry::File(*size));
                        }
                        InputEntry::Directory(name) => {
                            // put a new empty subtree in its place, if one didn't already exist
                            let TreeEntry::Directory(_) = tree.entry(name.as_str()).or_insert_with(|| TreeEntry::Directory(Tree::new())) else {
                                return Err(format!("tried to replace a file with a directory: {:?}", name).into());
                            };
                        }
                    }
                }
            }
        }
    }

    // when we're out of commands, then pretend we went all the way back up to the root to exit the whole thing
    Ok(UpwardCd::Root)
}

fn recurse_through<F>(t: &Tree, func: &mut F) -> u64
where
    F: FnMut(u64) -> (),
{
    let mut this_level = 0;
    for v in t.values() {
        match v {
            TreeEntry::File(size) => this_level += *size as u64,
            TreeEntry::Directory(subtree) => this_level += recurse_through(subtree, func),
        }
    }
    func(this_level);

    this_level
}
