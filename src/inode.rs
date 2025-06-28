use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::{get_current_byte_in_stdin, print, reset_allowed_backspaces};

lazy_static! {
    static ref INODE_TREE: Mutex<Inode> = Mutex::new(Inode::Directory(Directory {
        name: String::from("/"),
        children: Vec::new(),
    }));
}

#[derive(Clone)]
pub enum Inode {
    File(File),
    Directory(Directory),
    InputReciever(InputReciever),
    Outputter(Outputter),
}

pub enum InodeType {
    File,
    Directory,
    InputReciever,
    Outputter,
}

#[derive(Clone)]
pub struct File {
    name: String,
    size: u64,
    data: Vec<u8>,
}

#[derive(Clone)]
pub struct Directory {
    name: String,
    children: Vec<Inode>,
}

#[derive(Clone)]
pub struct InputReciever {
    name: String,
    on_input: fn(Vec<u8>),
}

#[derive(Clone)]
pub struct Outputter {
    name: String,
    on_output: fn() -> Vec<u8>,
}

impl Inode {
    pub fn get_type(&self) -> InodeType {
        match self {
            Inode::File(_) => InodeType::File,
            Inode::Directory(_) => InodeType::Directory,
            Inode::InputReciever(_) => InodeType::InputReciever,
            Inode::Outputter(_) => InodeType::Outputter,
        }
    }

    pub fn delete(path: &str) {
        let mut inode_tree = INODE_TREE.lock();
        if let Inode::Directory(directory) = &mut *inode_tree {
            directory.children.retain(|child| match child {
                Inode::File(file) => file.name != path,
                Inode::Directory(dir) => dir.name != path,
                Inode::InputReciever(stream) => stream.name != path,
                Inode::Outputter(stream) => stream.name != path,
            });
        }
    }

    // File operations
    pub fn read_file(&self) -> Option<Vec<u8>> {
        match self {
            Inode::File(file) => Some(file.data.clone()),
            _ => None,
        }
    }

    pub fn write_file(&mut self, data: Vec<u8>) {
        if let Inode::File(file) = self {
            file.data = data;
            file.size = file.data.len() as u64;
        }
    }

    pub fn create_file(name: &str, data: Vec<u8>) -> Inode {
        Inode::File(File {
            name: name.to_string(),
            size: data.len() as u64,
            data,
        })
    }

    // Directory operations
    pub fn list_directory(&self) -> Option<Vec<String>> {
        match self {
            Inode::Directory(dir) => Some(
                dir.children
                    .iter()
                    .map(|child| match child {
                        Inode::File(file) => file.name.clone(),
                        Inode::Directory(directory) => directory.name.clone(),
                        Inode::InputReciever(stream) => stream.name.clone(),
                        Inode::Outputter(stream) => stream.name.clone(),
                    })
                    .collect(),
            ),
            _ => None,
        }
    }

    pub fn create_directory(name: &str) -> Inode {
        Inode::Directory(Directory {
            name: name.to_string(),
            children: Vec::new(),
        })
    }

    // InputReciever operations
    pub fn write_inputreciever(&self, data: Vec<u8>) {
        match self {
            Inode::InputReciever(input_stream) => (input_stream.on_input)(data),
            _ => (),
        }
    }

    pub fn create_inputreciever(name: &str, on_input: fn(Vec<u8>)) -> Inode {
        Inode::InputReciever(InputReciever {
            name: name.to_string(),
            on_input,
        })
    }

    // Outputter operations
    pub fn read_outputter(&self) -> Option<Vec<u8>> {
        match self {
            Inode::Outputter(output_stream) => Some((output_stream.on_output)()),
            _ => None,
        }
    }

    pub fn create_outputter(name: &str, on_output: fn() -> Vec<u8>) -> Inode {
        Inode::Outputter(Outputter {
            name: name.to_string(),
            on_output,
        })
    }
}

// Initialize the device file system in /dev
pub fn init_dev() {
    let dev = Inode::Directory(Directory {
        name: String::from("dev"),
        children: vec![
            Inode::InputReciever(InputReciever {
                name: String::from("stdout"),
                on_input: |data| {
                    for byte in data {
                        print!("{}", String::from_utf8_lossy(&[byte]));
                    }
                    reset_allowed_backspaces();
                },
            }),
            Inode::InputReciever(InputReciever {
                name: String::from("serial0"),
                on_input: |data| {
                    crate::serial_print!("{}", String::from_utf8_lossy(&data));
                },
            }),
            Inode::Outputter(Outputter {
                name: String::from("stdin"),
                on_output: || {
                    let byte = get_current_byte_in_stdin();
                    vec![byte as u8]
                },
            }),
        ],
    });

    let mut inode_tree = INODE_TREE.lock();
    if let Inode::Directory(ref mut directory) = *inode_tree {
        directory.children.push(dev);
    }
}

// Retrieve an inode by path
pub fn get_inode(path: &str) -> Option<Inode> {
    let inode_tree = INODE_TREE.lock();
    let mut current_inode = &*inode_tree;
    for part in path.split("/") {
        if part == "" {
            continue;
        }
        match current_inode {
            Inode::Directory(directory) => {
                let mut found = false;
                for child in &directory.children {
                    match child {
                        Inode::Directory(dir) => {
                            if dir.name == part {
                                current_inode = child;
                                found = true;
                                break;
                            }
                        }
                        Inode::File(file) => {
                            if file.name == part {
                                current_inode = child;
                                found = true;
                                break;
                            }
                        }
                        Inode::InputReciever(input_stream) => {
                            if input_stream.name == part {
                                current_inode = child;
                                found = true;
                                break;
                            }
                        }
                        Inode::Outputter(output_stream) => {
                            if output_stream.name == part {
                                current_inode = child;
                                found = true;
                                break;
                            }
                        }
                    }
                }
                if !found {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(current_inode.clone())
}
