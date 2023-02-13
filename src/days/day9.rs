use std::{str::FromStr, collections::HashSet};

use crate::{solution::{AOCSolution, Labeled}, vec2::Vec2};

#[derive(Debug,Clone,Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}
impl Dir {
    pub fn delta(&self) -> Vec2<isize> {
        match self {
            Dir::Up => Vec2::new(0,1),
            Dir::Down => Vec2::new(0,-1),
            Dir::Left => Vec2::new(-1,0),
            Dir::Right => Vec2::new(1,0),
        }
    }
}


#[derive(Debug,Clone)]
pub struct Instructions(Vec<(Dir, i8)>);
impl FromStr for Instructions {
    type Err=();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ins:Vec<_> = s.lines().map(|l|{
            let Some((d,n)) = l.split_once(' ') else {
                return Err(())
            };
            let dir = match d {
                "U"=>Dir::Up,
                "D"=> Dir::Down,
                "L"=> Dir::Left,
                "R"=>Dir::Right,
                _=>return Err(())
            };

            let n = n.parse::<i8>().map_err(|_|{})?;
            Ok((dir,n))
        }).collect::<Result<_,_>>()?;
        Ok(Self(ins))
    }
}

pub fn tail_positions<const N:usize,I:IntoIterator<Item=(Dir,i8)>>(steps:I) -> HashSet<Vec2<isize>>
{
    let mut knots = [();N].map(|_|Vec2::<isize>::default());
    let mut tail_positions: HashSet<_> = HashSet::from_iter([Vec2::<isize>::default()]);
    for (dir,n) in steps {
        let delta = dir.delta();
        for _ in 0..n {
            knots[0] += delta;
            let mut knots_mut = knots.iter_mut();
            let mut prev_knot = knots_mut.next().unwrap();
            for other_knot in knots_mut {
                let diff = *prev_knot - *other_knot;

                let [x,y] = [diff.x,diff.y].map(isize::abs);

                if x==1 && y ==1 {
                    // Do nothing
                    break;
                } else if x == 0 { 
                    if y >= 2 {
                        other_knot.y += diff.y.signum();
                    }
                }else if y == 0 { 
                    if x >= 2 {
                        other_knot.x += diff.x.signum();
                    }
                } else {
                    other_knot.x += diff.x.signum();
                    other_knot.y += diff.y.signum();
                }
                prev_knot = other_knot;
            }
            // dbg!(knots);
            tail_positions.insert(knots[N-1]);
        }
    };
    tail_positions
}


pub struct Solution;
impl AOCSolution for Solution {
    type Input=Instructions;
    type Part1=Labeled<usize>;
    type Part2=Labeled<usize>;
    type Err = ();
    fn solve(input:Self::Input)->Result<(Self::Part1,Self::Part2),()> {
        // let mut tail_positions:HashSet<Vec2<isize>> = Default::default();

        // let mut tail = Vec2::default();
        // let mut head = Vec2::default();

        // tail_positions.insert(Default::default());
        // for (d,n) in input.0 {
        //     let delta = d.delta();
        //     for _ in 0..n {
        //         head+=delta;
                

        //         let diff = head - tail;

        //         let [x,y] = [diff.x,diff.y].map(isize::abs);

        //         if x==1 && y ==1 {
        //             // Do nothing
        //         } else if x == 0 { 
        //             if y >= 2 {
        //                 tail.y += diff.y.signum();
        //             }
        //         }else if y == 0 { 
        //             if x >= 2 {
        //                 tail.x += diff.x.signum();
        //             }
        //         } else {
        //             tail.x += diff.x.signum();
        //             tail.y += diff.y.signum();
        //         }
        //         tail_positions.insert(tail);
        //     }
        // };
        let len_2 = tail_positions::<2,_>(input.0.iter().cloned());
        let len_10 = tail_positions::<10,_>(input.0);
        Ok((
            (len_2.len(),"position").into(),
            (len_10.len(),"position").into()
        ))
    }
}