use std::{str::FromStr, collections::{HashMap, HashSet}, ops::{Range, ControlFlow}};

use crate::{solution::{Unsolved, AOCSolution}, matcher::{Matcher, FromStrMatcher}, or::Or};

#[derive(Debug,Clone,PartialEq, Eq)]
pub enum RangeInProgress {
    NoEndpoints,
    Start(usize),
    Full(Range<usize>)
}
impl RangeInProgress{
    pub fn full(self)->Option<Range<usize>>{
        if let RangeInProgress::Full(range) = self {
            Some(range)
        }else{
            None
        }
    }
}
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Step{
    Continue(usize),
    TurnLeft,
    TurnRight
}
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Map {
    row_ranges:Vec<Range<usize>>,
    col_ranges:Vec<Range<usize>>,
    walls:HashSet<(usize,usize)>,
    steps:Vec<Step>,
}
impl FromStr for Map {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_ranges = Vec::new();
        let mut col_ranges = Vec::new();
        let mut walls = HashSet::new();

        let (map,steps) = s.split_once("\n\n").ok_or(())?;

        for (row,row_str) in map.lines().enumerate(){
            let mut chars = row_str.char_indices();
            let first_non_space_in_row = loop {
                if let Some((column, c)) = chars.next() {
                    if c == '#' {
                        walls.insert((column,row));
                        break Some(column);
                    }else if c == '.' {
                        break Some(column);
                    }
                }else{
                    break None
                }
            };
            let Some(first_non_space_in_row) = first_non_space_in_row else{ continue;};

            let next_space_in_row = loop {
                if let Some(next) = chars.next() {
                    match next {
                        next@(_,' ')=>{
                            break Some(next.0);
                        }
                        (col,'#')=>{
                            walls.insert((col,row));
                        },
                        _=>{

                        }
                    }
                } else{
                    break None
                }
            };
            let next_space_in_row = next_space_in_row.unwrap_or(row_str.len());
            if let Some(additional) = next_space_in_row.checked_sub(col_ranges.len()) {
                col_ranges.extend(std::iter::repeat_with(||RangeInProgress::NoEndpoints).take(additional));
            }
            let row_range = first_non_space_in_row..next_space_in_row;
            for space_column in (0..first_non_space_in_row).chain(next_space_in_row..col_ranges.len()) {
                if let RangeInProgress::Start(start) = col_ranges[space_column]{
                    col_ranges[space_column] = RangeInProgress::Full(start..row);
                }
            };
            for tile_column in row_range.clone() {
                match &col_ranges[tile_column]{
                    RangeInProgress::NoEndpoints =>  {
                        col_ranges[tile_column] = RangeInProgress::Start(row);
                    },
                    RangeInProgress::Full(range) =>{
                        panic!("Tile after already completed column! {range:?}")
                    },
                    _=>{
                    }
                }
            };
            row_ranges.push(row_range);
        }
        let num_rows = row_ranges.len();
        let col_ranges = col_ranges.into_iter().map(|r|{
            match r {
                RangeInProgress::NoEndpoints => None,
                RangeInProgress::Start(s) => Some(s..num_rows),
                RangeInProgress::Full(r) => Some(r),
            }
        }).collect::<Option<Vec<_>>>().ok_or(())?;
        
        let steps = FromStrMatcher::<usize>::default().and('L'.or('R').and(FromStrMatcher::<usize>::default()).many::<Vec<_>>()).next_match(steps);
        let steps= steps.unwrap();

        let (first_num, (rest,_)) = steps.matched;

        let steps = std::iter::once(Step::Continue(first_num)).chain(rest.into_iter().flat_map(|(dir,cont)|{
            [
                match dir {
                    Or::Left(_)=>Step::TurnLeft,
                    Or::Right(_)=>Step::TurnRight,
                },
                Step::Continue(cont)
            ]
        }));
        Ok(Self{
            row_ranges,
            col_ranges,
            walls,
            steps:steps.collect()
        })
    }
}

#[repr(u8)]
#[derive(Debug,Clone,Copy,PartialEq, Eq)]
pub enum Facing {
    Right=0,
    Down=1,
    Left=2,
    Up=3
}

pub struct Solution;
impl AOCSolution for Solution {
    type Input=Map;
    type Part1=usize;
    type Part2=Unsolved;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        let mut pos = (input.row_ranges[0].start,0usize);
        let mut facing = Facing::Right;
        
        // let mut w = input.walls.iter().collect::<Vec<_>>();
        // w.sort();
        // println!("{:?}",w);
        for step in input.steps {
            // println!("{:?},{:?},{:?}",pos,facing,step);
            match step {
                Step::TurnLeft=> {
                    facing = match facing{
                        Facing::Right => Facing::Up,
                        Facing::Down => Facing::Right,
                        Facing::Left => Facing::Down,
                        Facing::Up => Facing::Left,
                    }
                },
                Step::TurnRight=> {
                    facing = match facing{
                        Facing::Right => Facing::Down,
                        Facing::Down => Facing::Left,
                        Facing::Left => Facing::Up,
                        Facing::Up => Facing::Right,
                    }
                },
                Step::Continue(n)=> {
                    fn n_or_walls<I:Iterator<Item=(usize,usize)>>(initial_pos:(usize,usize), n:usize, steps:I, walls:&HashSet<(usize,usize)>)->(usize,usize){
                        let stop_pos = steps.take(n).try_fold(initial_pos,|prev_pos,next_pos|{
                            if walls.contains(&next_pos) {
                                // println!("Ran into the wall at {next_pos:?}. Stopping at {prev_pos:?}");
                                ControlFlow::Break(prev_pos)
                            }else{
                                ControlFlow::Continue(next_pos)
                            }
                        });
                        match stop_pos {
                            ControlFlow::Continue(p)=>p,
                            ControlFlow::Break(p)=>p,
                        }
                    }
                    pos = match facing{
                        Facing::Right => {
                            let current_row = &input.row_ranges[pos.1];
                            n_or_walls(
                                pos,
                                n,
                                (pos.0+1..current_row.end).chain(current_row.clone().cycle()).map(|x|(x,pos.1)),
                                &input.walls
                            )
                        },
                        Facing::Down => {
                            let current_col =&input.col_ranges[pos.0];
                            n_or_walls(
                                pos,
                                n,
                                (pos.1+1..current_col.end).chain(current_col.clone().cycle()).map(|y|(pos.0,y)),
                                &input.walls
                            )
                        },
                        Facing::Left =>{
                            let current_row = &input.row_ranges[pos.1];
                            n_or_walls(
                                pos,
                                n,
                                ((current_row.start..pos.0).rev()).chain(current_row.clone().rev().cycle()).map(|x|(x,pos.1)),
                                &input.walls
                            )
                        },
                        Facing::Up => {
                            let current_col =&input.col_ranges[pos.0];
                            n_or_walls(
                                pos,
                                n,
                                ((current_col.start..pos.1).rev()).chain(current_col.clone().rev().cycle()).map(|y|(pos.0,y)),
                                &input.walls
                            )
                        },
                    }
                }
            }
        }
        Ok((4*(pos.0+1)+1000*(pos.1+1)+(facing as usize),Unsolved))
    }
}