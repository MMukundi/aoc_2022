use std::{str::FromStr, ops::{Index, IndexMut}, collections::HashSet, fmt::Debug};

use rayon::vec;

use crate::{solution::{Unsolved, AOCSolution, Labeled}, bitset::{BitSet, DefaultedBytes}};

use super::{day18::Pos, day9::Dir};

type BlizzardSquare = BitSet<Direction,DefaultedBytes::<1>>;

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Direction {
    Right=0,
    Down=1,
    Left=2,
    Up=3,
}
impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        value as _
    }
}
impl TryFrom<usize> for Direction {
    type Error=usize;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < 4 {
            Ok([
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Up
            ][value])
        }else{
            Err(value)
        }
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
pub enum Position {
    Entrance,
    Grid((usize,usize)),
    Exit
}
#[derive(Debug,Clone)]
pub enum PositionIter {
    Entrance,
    Grid {
        pos:(usize,usize),
        size:(usize,usize),
        remaining_directions:std::array::IntoIter<Direction,4>
    },
    Exit{size:(usize,usize),},
    Exhausted
}
impl Iterator for PositionIter {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            PositionIter::Entrance => {
                *self = PositionIter::Exhausted;
                Some(Position::Grid((0,0)))
            },
            PositionIter::Grid {size, pos, remaining_directions } => {
                let pos = remaining_directions.find_map(|d|{
                    match d {
                        Direction::Up => {
                            if let Some(j) = pos.1.checked_sub(1){
                                Some(Position::Grid((pos.0, j)))
                            }
                            else {
                                (pos.0 == 0).then_some(Position::Entrance)
                            }
                        },
                        Direction::Down => {
                            let new_j = pos.1+1;
                            if new_j<size.1{
                                Some(Position::Grid((pos.0, new_j)))
                            }
                            else {
                                size.0.checked_sub(1).and_then(|i|(pos.0 ==i).then_some(Position::Exit))
                            }
                        },
                        Direction::Left => {
                            pos.0.checked_sub(1).map(|i|Position::Grid((i, pos.1)))
                        },
                        Direction::Right => {
                            let new_i = pos.0+1;
                            (new_i < size.0).then_some(Position::Grid((new_i, pos.1)))
                        },
                    }
                });
                if pos.is_none() {
                    *self = Self::Exhausted;
                }
                pos
            },
            &mut PositionIter::Exit{size} => {
                *self = Self::Exhausted;
                size.0.checked_sub(1).zip(size.1.checked_sub(1)).map(Position::Grid)
            },
            PositionIter::Exhausted => None,
        }
    }
}
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct BlizzardGrid<T> {
    size:(usize,usize),
    // entrance:T,
    // exit:T,
    data:Vec<T>
}
impl BlizzardGrid<BlizzardSquare> {
    pub fn str(&self)->String{
        let wall = "#".repeat(self.size.0);
        let mid = self.data.chunks_exact(self.size.0).flat_map(|row|{
            std::iter::once('#').chain(row.iter().flat_map(|space|{
                let mut iter = space.into_iter();
                let Some(first) = iter.next() else {
                    return Some('.');
                };
                let None = iter.next() else {
                    let num = iter.count()+2;
                    return num.to_string().chars().next();
                };
                let Ok(first) = first else {
                    return None;
                };
                match first {
                    Direction::Right => Some('>'),
                    Direction::Down => Some('v'),
                    Direction::Left => Some('<'),
                    Direction::Up => Some('^'),
                }
            })).chain("#\n".chars())
        }).collect::<String>();
        format!("#.{wall}\n{mid}{wall}.#")
    }
}
impl <T> BlizzardGrid<T>{
    pub fn adjacents_to(&self,position:Position)->PositionIter{
        match position {
            Position::Entrance => PositionIter::Entrance,
            Position::Grid(pos) => {   
                if pos.0<self.size.0&&pos.1<self.size.1{
                    PositionIter::Grid { pos, size: self.size, remaining_directions: [
                        Direction::Down,
                        Direction::Right,
                        Direction::Up,
                        Direction::Left,
                    ].into_iter() }
                }else{
                    PositionIter::Exhausted
                }
            },
            Position::Exit => PositionIter::Exit { size: self.size },
        }
    }
    pub fn new<I:IntoIterator<Item=J>,J:IntoIterator<Item=T>>(data:I)->Self
        where I:Debug,J:Debug,T:Debug
    {
        let mut data_iters = data.into_iter();
        let first = data_iters.next();
        let Some(first) = first else {
            return Self {
                size:(0,0),
                data:Default::default(),
                // entrance,
                // exit
            }
        };
        let mut data_vec:Vec<_> = first.into_iter().collect();
        let mut size = (data_vec.len(),1);
        for iter in data_iters {
            data_vec.extend(iter);
            size.1+=1;
        };
        Self { size, /*entrance, exit,*/ data: data_vec }
    }
}
impl <T> Index<(usize,usize)> for BlizzardGrid<T>{
    type Output=T;

    #[inline]
    fn index(&self, (i,j): (usize,usize)) -> &Self::Output {
        &self.data[i+j*self.size.0]
    }
}
impl <T> IndexMut<(usize,usize)> for BlizzardGrid<T>{  
    #[inline]
    fn index_mut(&mut self, (i,j): (usize,usize)) -> &mut Self::Output {
        &mut self.data[i+j*self.size.0]
    }
}

impl FromStr for BlizzardGrid<BlizzardSquare> {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // Skip the entrance
        lines.next_back(); // Skip the exit

        Ok(BlizzardGrid::new( 
            lines.map(|line|{
                let mut chars = line.chars();
                chars.next(); // Skip the wall
                chars.next_back(); // Skip the wall
                chars.map(|c|{
                    let mut set =BitSet::default();
                    match c {
                        '<'=> {set.insert(Direction::Left);},
                        '>'=> {set.insert(Direction::Right);},
                        '^'=> {set.insert(Direction::Up);},
                        'v'=> {set.insert(Direction::Down);},
                        _=>{}
                    }
                    set
                })
            })
        ))
    }
}

#[inline]
fn inc_wrap(value:usize, max:usize)->usize {
    let new_value = value+1;
    if new_value < max {
        new_value
    }else {
        new_value - max
    }
}
#[inline]
fn dec_wrap(value:usize, max:usize)->usize {
    value.checked_sub(1).unwrap_or(max-1)
}

fn sim(start:Position,end:&Position,blizzards:&mut BlizzardGrid<BlizzardSquare>)->Option<usize> {
    let mut current_positions:HashSet<Position> = [start].into_iter().collect();
    (0..).find(|i|{
        // dbg!(&current_positions);
        // println!("{}",blizzards.str());
        if current_positions.contains(end) {
            true
        }else{
            current_positions=current_positions
                .iter()
                .filter(|&&p|{
                    if let Position::Grid(pos) = p {
                        blizzards[pos].is_empty()
                    }else {
                        true
                    }
                })
                .flat_map(|&p|blizzards.adjacents_to(p).chain(Some(p)))
                .collect();
            let mut new_blizzards = vec![
                BlizzardSquare::default();blizzards.data.len()
            ];
            for j in 0..blizzards.size.1 {
                for i in 0..blizzards.size.0 {
                    for dir in blizzards[(i,j)]{
                        let Ok(dir) = dir else {
                            continue;
                        };
                        let next_pos = match dir {
                            Direction::Right => (inc_wrap(i,blizzards.size.0),j),
                            Direction::Down => (i,inc_wrap(j,blizzards.size.1)),
                            Direction::Left => (dec_wrap(i,blizzards.size.0),j),
                            Direction::Up => (i,dec_wrap(j,blizzards.size.1)),
                        };
                        // println!("{:?} moved {dir:?} to {next_pos:?}",(i,j));
                        new_blizzards[next_pos.0+next_pos.1*blizzards.size.0].insert(dir);
                    }
                };
            }
            blizzards.data = new_blizzards;
            false
        }
        
    })
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=BlizzardGrid<BlizzardSquare>;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(mut blizzards:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let to_exit = sim(Position::Entrance,&Position::Exit,&mut blizzards);
        let to_start = sim(Position::Exit,&Position::Entrance,&mut blizzards);
        let to_exit_again = sim(Position::Entrance,&Position::Exit,&mut blizzards);
        to_exit.zip(to_start.zip(to_exit_again).map(|(a,b)|a+b)).map(|(a,b)|(
            (a,"minutes").into(),(a+b,"minutes").into()
        )).ok_or(())
    }
}