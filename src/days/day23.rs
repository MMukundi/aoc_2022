use std::{str::FromStr, collections::{HashSet, HashMap}};

use crate::{solution::{Unsolved, AOCSolution}, or::Or, days::day9::Dir};

type Num = i32;
type Pos = (Num,Num);

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Direction {
    North=0,
    South=1,
    West=2,
    East=3,
}
impl Direction{
    const fn get(&self) -> fn(Pos)->[Pos;3]{
        [
            |(x,y)|[(x,y-1),(x-1,y-1),(x+1,y-1)],
            |(x,y)|[(x,y+1),(x-1,y+1),(x+1,y+1)],
            |(x,y)|[(x-1,y),(x-1,y-1),(x-1,y+1)],
            |(x,y)|[(x+1,y),(x+1,y-1),(x+1,y+1)],
        ][*self as usize]
    }
    const fn adjacents(&self,(x,y):Pos) -> [Pos;3]{
        match self {
            Self::North=>[(x,y-1),(x-1,y-1),(x+1,y-1)],
            Self::South=>[(x,y+1),(x-1,y+1),(x+1,y+1)],
            Self::West=>[(x-1,y),(x-1,y-1),(x-1,y+1)],
            Self::East=>[(x+1,y),(x+1,y-1),(x+1,y+1)],
        }
    }
}

#[derive(Debug,Clone)]
pub struct InputStruct {
    elf_positions:HashSet<(Num,Num)>
}
impl FromStr for InputStruct {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elf_positions = s.lines().enumerate().flat_map(|(row,line)|{
            line.char_indices().filter_map(move |(col,c)|{
                (c == '#').then_some((col as _,row as _))
            })
        }).collect();
        Ok(Self{
            elf_positions
        })
    }
}

fn min_max<T:Ord+Clone,I:IntoIterator<Item=(T,T)>>(positions:I)->Option<((T,T),(T,T))>{
    let mut iter = positions.into_iter();
    let first = iter.next()?;
    let min_max = iter.fold((first.clone(),first), |(mut min,mut max),(x,y)|{
        if x<min.0 {
            min.0=x;
        } else if max.0 < x {
            max.0 = x;
        };

        if y<min.1 {
            min.1=y;
        } else if max.1 < y {
            max.1 = y;
        };
        (min,max)
    });
    Some(min_max)
}
fn display(positions:&HashSet<Pos>){
    let Some((min,max)) = min_max(positions.iter().cloned()) else {
        return;
    };
    let map = (min.1..=max.1).flat_map(|y|{
        (min.0..=max.0).map(move |x|{
            if positions.contains(&(x,y)) {
                '#'
            }else{
                '.'
            }
        }).chain(['\n'])
    }).collect::<String>();
    print!("----------\n{}",map);

}

fn round(positions:&HashSet<Pos>,order:[Direction;4])->(HashMap<Pos,(Pos,Vec<Pos>)>,HashSet<Pos>){
    let mut proposed_by:HashMap<Pos,(Pos,Vec<Pos>)> = Default::default();
    let mut new_positions:HashSet<Pos> = Default::default();
    for position in positions {
        if order.iter().all(|d|d.adjacents(*position).iter().all(|a|!positions.contains(a))){
            new_positions.insert(*position);
            continue;
        }
        let any_moves = order.clone().into_iter()
            // .map(|f|f(*position))
            .map(|d|d.adjacents(*position))
            .any(|adjs|{
                if adjs.iter().all(|a|!positions.contains(a)){
                    match proposed_by.entry(adjs[0]){
                        std::collections::hash_map::Entry::Occupied(mut o) => {
                            o.get_mut().1.push(*position)
                        },
                        std::collections::hash_map::Entry::Vacant(v) => {
                            v.insert((*position,Default::default()));
                        },
                    }
                    true
                }else{
                    false
                }
            });
        if !any_moves {
            new_positions.insert(*position);
        }
    }
    (proposed_by,new_positions)
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=InputStruct;
    type Part1=usize;
    type Part2=usize;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        const DIRECTION_ORDERS:[[Direction;4];4] = {
            use Direction::*;
            [
                [North, South, West,  East, ], 
                [South, West,  East,  North,],
                [West,  East,  North, South,],
                [East,  North, South, West, ],
            ]
            // [
            //     [North.get(), South.get(), West.get(),  East.get(), ],
            //     [South.get(), West.get(),  East.get(),  North.get(),],
            //     [West.get(),  East.get(),  North.get(), South.get(),],
            //     [East.get(),  North.get(), South.get(), West.get(), ],
            // ]
        };
        let mut orders = DIRECTION_ORDERS.into_iter().cycle();
        let mut positions = input.elf_positions.clone();
        // display(&positions);
        for order in orders.by_ref().take(10) {
            let (proposals,new_positions)= round(&positions,order);

            positions = new_positions;
            
            positions.extend(proposals.into_iter().flat_map(|(new_pos,(old_pos,others))|{
                if others.is_empty(){
                    Or::Left(Some(new_pos))
                }else{
                    Or::Right(Some(old_pos).into_iter().chain(others))
                }
            }).map(Or::collapse));
        }
        // display(&positions);
        let Some((min,max)) = min_max(positions.iter().cloned()) else {
            return Err(());
        };
        let empty = (min.0..=max.0)
            .flat_map(|x|(min.1..=max.1).map(move |y|(x,y)))
            .filter(|pos|!positions.contains(pos)).count();
        let done_round = orders.position(|order|{
            let (proposals,new_positions)= round(&positions,order);

            positions = new_positions;
            if proposals.is_empty() {
                true
            }else{
                positions.extend(proposals.into_iter().flat_map(|(new_pos,(old_pos,others))|{
                    if others.is_empty(){
                        Or::Left(Some(new_pos))
                    }else{
                        Or::Right(Some(old_pos).into_iter().chain(others))
                    }
                }).map(Or::collapse));
                false
            }
        }).map(|n|n+11);

        Ok((empty,done_round.unwrap()))
    }
}