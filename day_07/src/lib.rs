use prelude::log::debug;
use prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
enum Command {
    Cd(Component),
    Ls(Vec<Entry>),
}

#[derive(Debug)]
enum Component {
    Root,
    Parent,
    Path(String),
}

#[derive(Debug)]
enum Entry {
    File { size: u32, name: String },
    Directory(String),
}

use Command::*;
use Component::*;
use Entry::*;

#[wasm_bindgen]
pub struct Solution {
    commands: Vec<Command>,
}

#[wasm_bindgen]
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
                            entries.push(Directory(entry.strip_prefix("dir ").unwrap().to_owned()))
                        } else {
                            let (size, name) =
                                entry.split_once(' ').expect("there should be a space");
                            entries.push(File {
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
}
