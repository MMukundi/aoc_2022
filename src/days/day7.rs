use std::{str::FromStr, num::ParseIntError, sync::Arc, cell::RefCell, fmt::Debug, ops::Deref, collections::{BinaryHeap}};

use crate::solution::{AOCSolution};

type Ref<T> = Arc<RefCell<T>>;

#[derive(Debug,Clone)]
pub enum CdDirectory {
    Root,
    Parent,
    Directory(String)
}

#[derive(Clone)]
pub struct FileSystemDirectory {
    parent: Option<Ref<FileSystemDirectory>>,
    name: String,
    children: Vec<FileSystemItem>,
}
impl FileSystemDirectory {

    pub fn directory_sizes(&self,sizes:&mut BinaryHeap<(usize,String)>)->usize {
        let mut my_size = 0;
        for item in &self.children {
            match item {
                FileSystemItem::Directory(d)=>{
                    let dir = d.deref().borrow();
                    let dir_size = dir.directory_sizes(sizes);
                    my_size +=dir_size;
                },
                FileSystemItem::File { name: _, size } => {
                    my_size+=*size;
                }
            }
        };
        sizes.push((my_size,self.name.clone()));
        my_size
    }
}

impl Debug for FileSystemDirectory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = &mut f.debug_struct("FileSystemDirectory");
        if let Some(p)= &self.parent{
            let p = p.deref().borrow();
            ds = ds.field("parent", &p.name);
        }
        ds.field("name", &self.name)
        .field("children", &self.children)
        .finish()
    }
}

#[derive(Debug,Clone)]
pub enum FileSystemItem {
    Directory(Ref<FileSystemDirectory>),
    File {
        name: String,
        size: usize
    }
}

#[derive(Debug,Clone)]
pub enum TerminalCommand {
    ChangeDirectory(CdDirectory),
    List,
}

#[derive(Debug,Clone)]
pub enum TerminalLine {
    Command(TerminalCommand),
    ListedDirectory(String),
    ListedFile {
        name:String,
        size:usize
    }
}

#[derive(Debug,Clone)]
pub struct ElfTerminal {
    terminal_lines: Vec<TerminalLine>,
    root_directory: Ref<FileSystemDirectory>,
    current_directory: Ref<FileSystemDirectory>,
}

#[derive(Debug,Clone)]
pub enum ElfTerminalError {
    MissingCommand,
    UnknownCommand(String),
    MissingCdDirectory,
    MissingListedDirectory,
    ParseFileSizeError(ParseIntError),
    MissingFileName,
    EmptyLine,
}

impl FromStr for ElfTerminal {
    type Err = ElfTerminalError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let terminal_lines = s.lines().map(|l|{
            let mut split = l.split_whitespace();
            match split.next() {
                Some("$") => match split.next() {
                    Some("cd") => match split.next() {
                        Some(directory) => {
                            if directory == ".." {
                                Ok(TerminalLine::Command(
                                    TerminalCommand::ChangeDirectory(CdDirectory::Parent)
                                ))
                            }else if directory == "/" {
                                Ok(TerminalLine::Command(
                                    TerminalCommand::ChangeDirectory(CdDirectory::Root)
                                ))
                            }else {
                                Ok(TerminalLine::Command(
                                    TerminalCommand::ChangeDirectory(CdDirectory::Directory(directory.to_owned()))
                                ))
                            }
                        },
                        None => Err(ElfTerminalError::MissingCdDirectory)
                    },
                    Some("ls") => {
                        Ok(TerminalLine::Command(TerminalCommand::List))
                    },
                    Some(s) => Err(ElfTerminalError::UnknownCommand(s.to_owned())),
                    None => Err(ElfTerminalError::MissingCommand)
                },
                Some("dir") => match split.next() {
                    Some(d) => Ok(TerminalLine::ListedDirectory(d.to_owned())),
                    None => Err(ElfTerminalError::MissingListedDirectory)
                },
                Some(size) => {
                    let size = size.parse();
                    match size {
                        Ok(size)=>{
                            match split.next() {
                                Some(name) => Ok(TerminalLine::ListedFile{
                                    name:name.to_owned(),
                                    size
                                }),
                                None => Err(ElfTerminalError::MissingFileName)
                            }
                        },
                        Err(e)=>Err(ElfTerminalError::ParseFileSizeError(e))
                    }
                    
                },
                None => Err(ElfTerminalError::EmptyLine)
            }
        });
        let terminal_lines = terminal_lines.collect::<Result<_,_>>()?;
        let root = Arc::new(RefCell::new(FileSystemDirectory {
            parent:None,
            name:"/".to_owned(),
            children:Default::default()
        }));
        Ok(Self {
            terminal_lines,
            current_directory: root.clone(),
            root_directory: root,
        })
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=ElfTerminal;
    type Part1=usize;
    type Part2=usize;
    type Err = ();
    fn solve(mut terminal:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut lines = terminal.terminal_lines.iter().peekable();
        while let Some(TerminalLine::Command(c)) = lines.next(){ 
                match c {
                    TerminalCommand::ChangeDirectory(d)=>{
                        match d {
                            CdDirectory::Parent => {
                                let parent = terminal.current_directory.deref().borrow().parent.clone();
                                if let Some(parent) = parent {
                                    terminal.current_directory = parent;
                                }else{
                                    panic!("No parent!")
                                }
                            },
                            CdDirectory::Root => {
                                let root = terminal.root_directory.clone();
                                terminal.current_directory=root;
                            }
                            CdDirectory::Directory(target_dir_name) => {
                                let child = terminal.current_directory.deref().borrow().children.iter().find_map(|item|{
                                    if let FileSystemItem::Directory(other_dir) = item {
                                        if target_dir_name == &other_dir.deref().borrow().name {
                                            return Some(other_dir.clone());
                                        }
                                    }
                                    None
                                }).clone();
                                if let Some(child_dir) = child{
                                    terminal.current_directory = child_dir.clone();
                                }else{
                                    panic!("No child {target_dir_name:?}!")
                                }
                            }
                        }
                    },
                    TerminalCommand::List => {
                        let current_dir_children = &mut terminal.current_directory.borrow_mut().children;
                        loop {
                            match lines.peek(){
                                Some(TerminalLine::ListedFile { name, size }) => {
                                    lines.next();
                                    current_dir_children.push(FileSystemItem::File { name:name.clone(), size:*size })
                                }
                                Some(TerminalLine::ListedDirectory(dir_name)) => {
                                    lines.next();
                                    current_dir_children.push(FileSystemItem::Directory(Arc::new(RefCell::new(FileSystemDirectory {
                                        parent:Some(terminal.current_directory.clone()),
                                        name:dir_name.to_owned(),
                                        children:Default::default()
                                    }))));
                                },
                                Some(_) => break,
                                None => break
                            }
                        }
                    }
                }
            };
        // dbg!(&terminal);
        let mut sizes = Default::default();
        let root_dir_size = terminal.root_directory.deref().borrow().directory_sizes(&mut sizes);
        let needed = 30000000-(70000000-root_dir_size);
        Ok((
            sizes.iter().map(|(s,_)|s).filter(|&&n|n<=100000).sum(),
            sizes.into_sorted_vec().into_iter().find_map(|(size,_name)|{
                if size>= needed {
                    Some(size)
                }else{
                    None
                }
            }).unwrap()
        ))
    }
}