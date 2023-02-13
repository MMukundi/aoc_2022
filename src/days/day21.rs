use std::{str::FromStr, collections::HashMap};

use crate::solution::{Unsolved, AOCSolution};

type Num = u64;
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(Debug,Clone)]
pub enum Mathkey {
    Operate(Operation,usize,usize),
    Yell(Num)
}

#[derive(Debug,Clone)]
pub struct InputStruct {
    root_mathkey:usize,
    humn_mathkey:usize,
    mathkeys: Vec<Mathkey>,
}

#[derive(Debug,PartialEq, Eq)]
pub enum Side{
    Left,
    Right
}

impl InputStruct {
    pub fn eval(&self,mathkey:usize) -> Num {
        match &self.mathkeys[mathkey] {
            Mathkey::Operate(op, l,r) => {
                let (l,r)=(self.eval(*l),self.eval(*r));
                match op {
                    Operation::Add => l+r,
                    Operation::Sub => l-r,
                    Operation::Mul => l*r,
                    Operation::Div => l/r,
                }
            },
            &Mathkey::Yell(num) => num,
        }
    }
    pub fn path_to(&self,dest:usize, current_mathkey:usize,mut current_path: Vec<Side>)->Result<Vec<Side>,Vec<Side>>{
        if current_mathkey == dest {
            return Ok(current_path);
        }
        match &self.mathkeys[current_mathkey] {
            Mathkey::Operate(op, l,r) => {
                current_path.push(Side::Left);
                self.path_to(dest, *l,current_path).or_else(|mut err_path|{
                    if let Some(last) = err_path.last_mut(){
                        *last = Side::Right;
                    }
                    self.path_to(dest, *r,err_path)
                }).map_err(|mut e|{
                    e.pop();
                    e
                })
            },
            Mathkey::Yell(num) => Err(current_path),
        }
    }
    pub fn solve_for(&self, mathkey:usize,mathkey_to_solve_for:usize)->Option<Num>{
        let path = self.path_to(mathkey_to_solve_for, mathkey, Vec::new());
        let Ok(mut path) = path else {
            return None;
        };
        let first = path.remove(0);
        let Mathkey::Operate(_, l,r) = self.mathkeys[mathkey] else {
            return None; // If root is not a binary
        };
        let (mut target,mut search_node) = if first == Side::Left {
            (self.eval(r),l)
        }else {
            (self.eval(l),r)
        };
        for step in path {
            let Mathkey::Operate(op, l,r) = self.mathkeys[search_node] else {
                return None; // If any step is not a binary
            }; 
            search_node = match step {
                Side::Left => l,
                Side::Right => r,
            };
            match (op,step) {
                (Operation::Add,Side::Left) => {
                    target -= self.eval(r)
                },
                (Operation::Add,Side::Right) => {
                    target -= self.eval(l)
                },
                (Operation::Mul,Side::Left) => {
                    target /= self.eval(r)
                },
                (Operation::Mul,Side::Right) => {
                    target /= self.eval(l)
                },
                (Operation::Sub,Side::Left) => {
                    target += self.eval(r)
                },
                (Operation::Sub,Side::Right) => {
                    target = self.eval(l)-target;
                },
                (Operation::Div,Side::Left) => {
                    target *= self.eval(r)
                },
                (Operation::Div,Side::Right) => {
                    target = self.eval(l)/target;
                },
            }
        };
        Some(target)
    }
}
impl FromStr for InputStruct {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = Vec::new();
        let n = &mut nodes;
        let mut node_index_by_name = HashMap::new();

        macro_rules! id_for_name {
            ($name:expr) => {
                *node_index_by_name.entry($name).or_insert_with(||{
                    let next_id = nodes.len();
                    nodes.push(None);
                    next_id
                })
            };
        }

        for line in s.lines() {
            let name = &line[..4];
            let index = id_for_name!(name);

            let mut rest = line[6..].split_whitespace();
            let first = rest.next().ok_or(())?;
            if let Some(operator) = rest.next() {
                let second = rest.next().ok_or(())?;
                let left_index = id_for_name!(first);
                let right_index = id_for_name!(second);
                let op = match operator {
                    "*" => Operation::Mul,
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "/" => Operation::Div,
                    _ => return Err(())
                };
                nodes[index] = Some(Mathkey::Operate(op, left_index, right_index));
            } else if let Ok(num) = first.parse::<Num>() {
                nodes[index] = Some(Mathkey::Yell(num));
            } else {
                panic!("Hm?")
            };
        };
        node_index_by_name.get("root").zip(node_index_by_name.get("humn")).zip(nodes.into_iter().collect::<Option<Vec<_>>>()).ok_or(()).map(move |((root_mathkey,humn_mathkey),mathkeys)|{
            Self{ 
                root_mathkey:*root_mathkey,
                humn_mathkey:*humn_mathkey,
                mathkeys
            }
        })
    }
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=InputStruct;
    type Part1=Num;
    type Part2=Num;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        Ok((input.eval(input.root_mathkey),input.solve_for(input.root_mathkey,input.humn_mathkey).unwrap()))
    }
}