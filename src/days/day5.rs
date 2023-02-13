
use std::fmt::Debug;

use std::str::FromStr;

use crate::array::{get_two_mut, next_chunk};

use crate::solution::{AOCSolution};


#[derive(Debug,Clone)]
pub struct ElfCrates{
    stacks: Vec<Vec<char>>,
    instructions:Vec<(u8,u8,u8)>
}
impl FromStr for ElfCrates {
    type Err=();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (setup,instructions) = s.split_once("\n\n").ok_or(())?;
        let mut lines = setup.lines().rev();
        let num = lines.next().map(|c|c.chars().filter(|c|!c.is_whitespace()).count()).unwrap_or_default();
        let mut stacks = vec![vec![];num];
        for line in lines {
            let mut lines = line.chars();
            let mut stacks_mut = stacks.iter_mut();
            
            if let Some(s) = stacks_mut.next(){
                if let Ok(['[',first_c,']']) = next_chunk(&mut lines) { // if not blank
                    s.push(first_c);
                }
            }

            for stack in stacks_mut {
                if let Ok([_,'[',c,']']) = next_chunk(&mut lines) { // if not blank
                    stack.push(c);
                }
            }
        };
        dbg!(&stacks);
        let instructions = instructions.lines().map( |l|{
            let mut split = l.split_whitespace();
            split.next();
            let mut step = split.step_by(2);
            let Some(a) = step.next() else {return Err(())};
            let Some(b) = step.next() else {return Err(())};
            let Some(c) = step.next() else {return Err(())};
            let a = a.parse::<u8>().map_err(|_|{}).map_err(|_|{})?;
            let b = b.parse::<u8>().map_err(|_|{}).map_err(|_|{})?;
            let c = c.parse::<u8>().map_err(|_|{}).map_err(|_|{})?;
            Ok((a,b-1,c-1))
        }).collect::<Result<Vec<_>,_>>().map_err(|_|{})?;
        Ok(Self {
            instructions,
            stacks
        })
    }
}



pub struct Solution;
impl AOCSolution for Solution {
    type Input=ElfCrates;
    type Part1=String;
    type Part2=String;
    type Err = ();
    fn solve(crates:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut mover_9000_stacks = crates.stacks.clone();
        for &(count,from,to) in &crates.instructions {
            if let Some((from,to)) = get_two_mut(&mut mover_9000_stacks, from as usize, to as usize) {
                let from_len = from.len();
                to.extend(from.drain((from_len-count as usize)..).rev());
            }
        }

        let mut mover_9001_stacks = crates.stacks;
        for &(count,from,to) in &crates.instructions {
            if let Some((from,to)) = get_two_mut(&mut mover_9001_stacks, from as usize, to as usize) {
                let from_len = from.len();
                to.extend(from.drain((from_len-count as usize)..));
            }
        }
        Ok((
            mover_9000_stacks.iter().filter_map(|s|s.last()).collect(),
            mover_9001_stacks.iter().filter_map(|s|s.last()).collect(),
        ))
    }
}
//12156
#[cfg(test)]
mod test {
    #[test]
    fn test(){

    }
}